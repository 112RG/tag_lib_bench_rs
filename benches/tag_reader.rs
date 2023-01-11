use criterion::{criterion_group, criterion_main, Criterion};
use lofty::{flac::FlacFile, Accessor, AudioFile, Probe, TaggedFileExt};
use metaflac::{block::VorbisComment, Tag};
use std::path::Path;
pub struct AudioMetadata {
    pub name: String,
    pub album: String,
    pub album_artist: String,
    pub year: u32,
    pub track: u32,
    pub path: String,
    pub lossless: bool,
    pub duration: u64,
}

pub fn get_metadata_metaflac(path: String) -> AudioMetadata {
    let tag = Tag::read_from_path(&path).unwrap();
    let vorbis: &VorbisComment = tag.vorbis_comments().unwrap();

    let mut stream_info = tag.get_blocks(metaflac::BlockType::StreamInfo);
    let duration = match stream_info.next() {
        Some(metaflac::Block::StreamInfo(s)) => Some(s.total_samples as u64 / s.sample_rate as u64),
        _ => None,
    };
    let metadata: AudioMetadata = AudioMetadata {
        name: vorbis
            .title()
            .map(|v| v[0].clone())
            .unwrap_or_else(|| "FAILED TO READ TITLE DEAFTONE".to_string()),
        album: vorbis
            .album()
            .map(|v| v[0].clone())
            .unwrap_or_else(|| "FAILED TO READ ALBUM DEAFTONE".to_string()),
        album_artist: match vorbis.album_artist().map(|v| v[0].clone()) {
            Some(e) => e,
            None => vorbis
                .artist()
                .map(|v| v[0].clone())
                .unwrap_or_else(|| "FAILED TO READ ARTIST DEAFTONE".to_string()),
        },
        year: get_year(vorbis),
        track: vorbis.track().unwrap_or(0),
        path,
        lossless: true,
        duration: duration.unwrap_or_default(),
    };
    metadata
}

fn get_year(vorbis: &VorbisComment) -> u32 {
    let original_year: String = vorbis
        .comments
        .get("ORIGINALYEAR")
        .and_then(|d| d[0].parse::<String>().ok())
        .unwrap_or_default();

    let date: String = vorbis
        .comments
        .get("DATE")
        .and_then(|d| d[0].parse::<String>().ok())
        .unwrap_or_default();
    let year: String = vorbis
        .comments
        .get("YEAR")
        .and_then(|d| d[0].parse::<String>().ok())
        .unwrap_or_default();

    if year.chars().count() >= 4 {
        return parse_year(year);
    } else if date.chars().count() >= 4 {
        return parse_year(date);
    } else if original_year.chars().count() >= 4 {
        return parse_year(original_year);
    } else {
        return 0;
    }
}

fn parse_year(mut year: String) -> u32 {
    if year.chars().count() == 10 {
        year.truncate(4);
        return year.parse::<u32>().unwrap_or_default();
    } else {
        return year.parse::<u32>().unwrap_or_default();
    }
}

pub fn get_metadata_lofty(path: String) -> AudioMetadata {
    let tagged_file = Probe::open(&path)
        .expect("ERROR: Bad path provided!")
        .read()
        .expect("ERROR: Failed to read file!");
    let tag = match tagged_file.primary_tag() {
        Some(primary_tag) => primary_tag,
        None => tagged_file.first_tag().expect("ERROR: No tags found!"),
    };

    let properties = tagged_file.properties();

    let duration = properties.duration();

    let metadata: AudioMetadata = AudioMetadata {
        name: tag.title().unwrap().to_string(),
        track: tag.track().unwrap(),
        album: tag.album().unwrap().to_string(),
        album_artist: tag
            .get_string(&lofty::ItemKey::AlbumArtist)
            .unwrap_or("None")
            .to_owned(),
        year: tag.year().unwrap(),
        path,
        lossless: true,
        duration: duration.as_secs(),
    };

    metadata
}

pub fn get_metadata_lofty_specify_type(path: String) -> AudioMetadata {
    let file = std::fs::File::open(&path).unwrap();

    let flac = FlacFile::read_from(
        std::io::BufReader::new(file).get_mut(),
        lofty::ParseOptions::new(),
    )
    .unwrap();
    let vorbis = flac.vorbis_comments().unwrap();
    let properties = flac.properties();
    let duration = properties.duration();

    let metadata: AudioMetadata = AudioMetadata {
        name: vorbis.title().unwrap().to_string(),
        track: vorbis.track().unwrap(),
        album: vorbis.album().unwrap().to_string(),
        album_artist: vorbis.get(&String::from("ALBUMARTIST")).unwrap().to_owned(),
        year: vorbis.year().unwrap(),
        path,
        lossless: true,
        duration: duration.as_secs(),
    };

    metadata
}

fn tag_reader(c: &mut Criterion) {
    let mut group = c.benchmark_group("Flac Tag Reader");
    group.bench_function(stringify!("metaflac"), |b| {
        b.iter(|| {
            get_metadata_metaflac(Path::new("./full_test.flac").to_string_lossy().to_string())
        })
    });
    group.bench_function(stringify!("lofty"), |b| {
        b.iter(|| get_metadata_lofty(Path::new("./full_test.flac").to_string_lossy().to_string()))
    });
    group.bench_function(stringify!("lofty specify file type"), |b| {
        b.iter(|| {
            get_metadata_lofty_specify_type(
                Path::new("./full_test.flac").to_string_lossy().to_string(),
            )
        })
    });
    group.finish();
}

criterion_group!(benches, tag_reader);
criterion_main!(benches);
