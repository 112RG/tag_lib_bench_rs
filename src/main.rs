use lofty::{Accessor, AudioFile, Probe, TaggedFileExt};

use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, MetadataRevision, StandardTagKey};
use symphonia::core::probe::Hint;
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AudioMetadata {
    pub name: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub year: u32,
    pub disc_number: String,
    pub composer: String,
    pub track: u32,
    pub path: String,
    pub lossless: bool,
    pub duration: u64,
    pub genres: String,
    pub comment: String,
}

impl Default for AudioMetadata {
    fn default() -> AudioMetadata {
        AudioMetadata {
            name: String::from(""),
            artist: String::from(""),
            composer: String::from(""),
            disc_number: String::from(""),
            album: String::from(""),
            album_artist: String::from(""),
            year: 0,
            track: 0,
            path: String::from(""),
            lossless: false,
            duration: 0,
            genres: String::from(""),
            comment: String::from(""),
        }
    }
}
pub fn main() {
    let path: String = "./full_test.flac".to_string();
    let src = std::fs::File::open(&path).expect("failed to open media");
    let mss = MediaSourceStream::new(Box::new(src), Default::default());
    let mut hint = Hint::new();
    hint.with_extension("flac");
    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();
    let mut probed = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("unsupported format");

    if let Some(metadata_rev) = probed.format.metadata().current() {
        let metadata = get_tags(&metadata_rev);
        println!("{:?}", metadata);
        if probed.metadata.get().as_ref().is_some() {
            println!("tags that are part of the container format are preferentially printed.");
            println!("not printing additional tags that were found while probing.");
        }
    } else if let Some(metadata_rev) = probed.metadata.get().as_ref().and_then(|m| m.current()) {
        let metadata = get_tags(&metadata_rev);
        println!("{:?}", metadata);
    }
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
                StandardTagKey::Artist => metadata.artist = tag.value.to_string(),
                StandardTagKey::Bpm => todo!(),
                StandardTagKey::Comment => metadata.comment = tag.value.to_string(),
                StandardTagKey::Compilation => todo!(),
                StandardTagKey::Composer => metadata.composer = tag.value.to_string(),
                StandardTagKey::Conductor => todo!(),
                StandardTagKey::ContentGroup => todo!(),
                StandardTagKey::Copyright => todo!(),
                StandardTagKey::Date => (),
                StandardTagKey::Description => todo!(),
                StandardTagKey::DiscNumber => metadata.disc_number = tag.value.to_string(),
                StandardTagKey::DiscSubtitle => todo!(),
                StandardTagKey::DiscTotal => todo!(),
                StandardTagKey::EncodedBy => todo!(),
                StandardTagKey::Encoder => todo!(),
                StandardTagKey::EncoderSettings => todo!(),
                StandardTagKey::EncodingDate => todo!(),
                StandardTagKey::Engineer => todo!(),
                StandardTagKey::Ensemble => todo!(),
                StandardTagKey::Genre => metadata.genres = tag.value.to_string(),
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
