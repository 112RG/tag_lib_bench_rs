use criterion::{criterion_group, criterion_main, Criterion};
use lofty::{flac::FlacFile, Accessor, AudioFile, Probe, TaggedFileExt};
use metaflac::{block::VorbisComment, Tag};
use std::path::Path;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, MetadataRevision, StandardTagKey};
use symphonia::core::probe::Hint;

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
impl Default for AudioMetadata {
    fn default() -> AudioMetadata {
        AudioMetadata {
            name: String::from(""),
            album: String::from(""),
            album_artist: String::from(""),
            year: 0,
            track: 0,
            path: String::from(""),
            lossless: false,
            duration: 0,
        }
    }
}
pub fn get_metadata_metaflac_reader(path: String) -> AudioMetadata {
    let file = std::fs::File::open(&path).unwrap();
    let mut reader = std::io::BufReader::new(file);
    let tag = Tag::read_from(&mut reader).unwrap();
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

pub fn get_metadata_symphonia(path: String) -> AudioMetadata {
    let src = std::fs::File::open(&path).expect("failed to open media");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("flac");
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();
    let mut probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    return if let Some(metadata_rev) = probed.format.metadata().current() {
        get_tags(&metadata_rev)
    /*         if probed.metadata.get().as_ref().is_some() {
        println!("tags that are part of the container format are preferentially printed.");
        println!("not printing additional tags that were found while probing.");
    } */
    } else if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
        get_tags(&metadata_rev)
    } else {
        AudioMetadata::default()
    };
}
fn get_tags(metadata_rev: &MetadataRevision) -> AudioMetadata {
    let tags = metadata_rev.tags();
    let mut metadata: AudioMetadata = AudioMetadata::default();
    for tag in tags.iter() {
        if let Some(std_key) = tag.std_key {
            match std_key {
                StandardTagKey::Album => metadata.album = tag.value.to_string(),
                StandardTagKey::AcoustidFingerprint => todo!(),
                StandardTagKey::AcoustidId => todo!(),
                StandardTagKey::AlbumArtist => metadata.album_artist = tag.value.to_string(),
                StandardTagKey::Arranger => todo!(),
                StandardTagKey::Artist => (),
                StandardTagKey::Bpm => todo!(),
                StandardTagKey::Comment => (),
                StandardTagKey::Compilation => todo!(),
                StandardTagKey::Composer => (),
                StandardTagKey::Conductor => todo!(),
                StandardTagKey::ContentGroup => todo!(),
                StandardTagKey::Copyright => todo!(),
                StandardTagKey::Date => (),
                StandardTagKey::Description => todo!(),
                StandardTagKey::DiscNumber => (),
                StandardTagKey::DiscSubtitle => todo!(),
                StandardTagKey::DiscTotal => todo!(),
                StandardTagKey::EncodedBy => todo!(),
                StandardTagKey::Encoder => todo!(),
                StandardTagKey::EncoderSettings => todo!(),
                StandardTagKey::EncodingDate => todo!(),
                StandardTagKey::Engineer => todo!(),
                StandardTagKey::Ensemble => todo!(),
                StandardTagKey::Genre => (),
                StandardTagKey::IdentAsin => todo!(),
                StandardTagKey::IdentBarcode => todo!(),
                StandardTagKey::IdentCatalogNumber => todo!(),
                StandardTagKey::IdentEanUpn => todo!(),
                StandardTagKey::IdentIsrc => todo!(),
                StandardTagKey::IdentPn => todo!(),
                StandardTagKey::IdentPodcast => todo!(),
                StandardTagKey::IdentUpc => todo!(),
                StandardTagKey::Label => todo!(),
                StandardTagKey::Language => todo!(),
                StandardTagKey::License => todo!(),
                StandardTagKey::Lyricist => todo!(),
                StandardTagKey::Lyrics => todo!(),
                StandardTagKey::MediaFormat => todo!(),
                StandardTagKey::MixDj => todo!(),
                StandardTagKey::MixEngineer => todo!(),
                StandardTagKey::Mood => todo!(),
                StandardTagKey::MovementName => todo!(),
                StandardTagKey::MovementNumber => todo!(),
                StandardTagKey::MusicBrainzAlbumArtistId => todo!(),
                StandardTagKey::MusicBrainzAlbumId => todo!(),
                StandardTagKey::MusicBrainzArtistId => todo!(),
                StandardTagKey::MusicBrainzDiscId => todo!(),
                StandardTagKey::MusicBrainzGenreId => todo!(),
                StandardTagKey::MusicBrainzLabelId => todo!(),
                StandardTagKey::MusicBrainzOriginalAlbumId => todo!(),
                StandardTagKey::MusicBrainzOriginalArtistId => todo!(),
                StandardTagKey::MusicBrainzRecordingId => todo!(),
                StandardTagKey::MusicBrainzReleaseGroupId => todo!(),
                StandardTagKey::MusicBrainzReleaseStatus => todo!(),
                StandardTagKey::MusicBrainzReleaseTrackId => todo!(),
                StandardTagKey::MusicBrainzReleaseType => todo!(),
                StandardTagKey::MusicBrainzTrackId => todo!(),
                StandardTagKey::MusicBrainzWorkId => todo!(),
                StandardTagKey::Opus => todo!(),
                StandardTagKey::OriginalAlbum => todo!(),
                StandardTagKey::OriginalArtist => todo!(),
                StandardTagKey::OriginalDate => todo!(),
                StandardTagKey::OriginalFile => todo!(),
                StandardTagKey::OriginalWriter => todo!(),
                StandardTagKey::Owner => todo!(),
                StandardTagKey::Part => todo!(),
                StandardTagKey::PartTotal => todo!(),
                StandardTagKey::Performer => todo!(),
                StandardTagKey::Podcast => todo!(),
                StandardTagKey::PodcastCategory => todo!(),
                StandardTagKey::PodcastDescription => todo!(),
                StandardTagKey::PodcastKeywords => todo!(),
                StandardTagKey::Producer => todo!(),
                StandardTagKey::PurchaseDate => todo!(),
                StandardTagKey::Rating => todo!(),
                StandardTagKey::ReleaseCountry => todo!(),
                StandardTagKey::ReleaseDate => todo!(),
                StandardTagKey::Remixer => todo!(),
                StandardTagKey::ReplayGainAlbumGain => todo!(),
                StandardTagKey::ReplayGainAlbumPeak => todo!(),
                StandardTagKey::ReplayGainTrackGain => todo!(),
                StandardTagKey::ReplayGainTrackPeak => todo!(),
                StandardTagKey::Script => todo!(),
                StandardTagKey::SortAlbum => todo!(),
                StandardTagKey::SortAlbumArtist => todo!(),
                StandardTagKey::SortArtist => todo!(),
                StandardTagKey::SortComposer => todo!(),
                StandardTagKey::SortTrackTitle => todo!(),
                StandardTagKey::TaggingDate => todo!(),
                StandardTagKey::TrackNumber => (),
                StandardTagKey::TrackSubtitle => todo!(),
                StandardTagKey::TrackTitle => metadata.name = tag.value.to_string(),
                StandardTagKey::TrackTotal => todo!(),
                StandardTagKey::TvEpisode => todo!(),
                StandardTagKey::TvEpisodeTitle => todo!(),
                StandardTagKey::TvNetwork => todo!(),
                StandardTagKey::TvSeason => todo!(),
                StandardTagKey::TvShowTitle => todo!(),
                StandardTagKey::Url => todo!(),
                StandardTagKey::UrlArtist => todo!(),
                StandardTagKey::UrlCopyright => todo!(),
                StandardTagKey::UrlInternetRadio => todo!(),
                StandardTagKey::UrlLabel => todo!(),
                StandardTagKey::UrlOfficial => todo!(),
                StandardTagKey::UrlPayment => todo!(),
                StandardTagKey::UrlPodcast => todo!(),
                StandardTagKey::UrlPurchase => todo!(),
                StandardTagKey::UrlSource => todo!(),
                StandardTagKey::Version => todo!(),
                StandardTagKey::Writer => todo!(),
            }
        }
    }
    return metadata;
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
    let mut reader = std::io::BufReader::new(file);
    let flac = FlacFile::read_from(&mut reader, lofty::ParseOptions::new()).unwrap();
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

pub fn get_metadata_taglib_rust(path: String) -> AudioMetadata {
    let file = taglib::File::new(&path).unwrap();
    let tags = file.tag().unwrap();
    let metadata: AudioMetadata = AudioMetadata {
        name: tags.title().unwrap().to_string(),
        track: tags.track().unwrap(),
        album: tags.album().unwrap().to_string(),
        album_artist: tags.artist().unwrap(),
        year: tags.year().unwrap(),
        path,
        lossless: true,
        duration: file.audioproperties().unwrap().length().into(),
    };

    metadata
}

fn tag_reader(c: &mut Criterion) {
    let mut group = c.benchmark_group("Flac Tag Reader");

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
    group.bench_function(stringify!("symphonia"), |b| {
        b.iter(|| {
            get_metadata_symphonia(Path::new("./full_test.flac").to_string_lossy().to_string())
        })
    });
    group.bench_function(stringify!("metaflac"), |b| {
        b.iter(|| {
            get_metadata_metaflac(Path::new("./full_test.flac").to_string_lossy().to_string())
        })
    });
    group.bench_function(stringify!("metaflac read_from"), |b| {
        b.iter(|| {
            get_metadata_metaflac_reader(
                Path::new("./full_test.flac").to_string_lossy().to_string(),
            )
        })
    });
    group.bench_function(stringify!("taglib-rust"), |b| {
        b.iter(|| {
            get_metadata_taglib_rust(Path::new("./full_test.flac").to_string_lossy().to_string())
        })
    });
    group.finish();
}

criterion_group!(benches, tag_reader);
criterion_main!(benches);
