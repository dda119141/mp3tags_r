use crate::meta_entry::MetaEntry;

/// Frame mapping for ID3v2.3 and ID3v2.4 (4-character frame IDs)
pub mod v3_v4 {
    use super::*;
    
    /// HashMap for ID3v2.3/v2.4 frame mappings
    use phf::{phf_map, Map};

    static FRAME_MAP: Map<&'static str, &'static str> = phf_map! {
        "Title" => "TIT2",
        "Artist" => "TPE1",
        "Album" => "TALB",
        "Year" => "TYER",
        "Genre" => "TCON",
        "Comment" => "COMM",
        "Composer" => "TCOM",
        "Track" => "TRCK",
        "Date" => "TDAT",
        "TextWriter" => "TEXT",
        "AudioEncryption" => "AENC",
        "Language" => "TLAN",
        "Time" => "TIME",
        "OriginalFilename" => "TOFN",
        "FileType" => "TFLT",
        "BandOrchestra" => "TPE2",
        "AttachedPicture" => "APIC",
        "AudioSeekPointIndex" => "ASPI",
        "CommercialFrame" => "COMR",
        "EncryptionMethodRegistration" => "ENCR",
        "Equalisation2" => "EQU2",
        "EventTimingCodes" => "ETCO",
        "GeneralEncapsulatedObject" => "GEOB",
        "GroupIdentificationRegistration" => "GRID",
        "LinkedInformation" => "LINK",
        "MusicCDIdentifier" => "MCDI",
        "MPEGLocationLookupTable" => "MLLT",
        "OwnershipFrame" => "OWNE",
        "PrivateFrame" => "PRIV",
        "PlayCounter" => "PCNT",
        "Popularimeter" => "POPM",
        "PositionSynchronisationFrame" => "POSS",
        "RecommendedBufferSize" => "RBUF",
        "RelativeVolumeAdjustment2" => "RVA2",
        "Reverb" => "RVRB",
        "SeekFrame" => "SEEK",
        "SignatureFrame" => "SIGN",
        "SynchronisedLyricText" => "SYLT",
        "SynchronisedTempoCodes" => "SYTC",
        "BeatsPerMinute" => "TBPM",
        "CopyrightMessage" => "TCOP",
        "EncodingTime" => "TDEN",
        "PlaylistDelay" => "TDLY",
        "OriginalReleaseTime" => "TDOR",
        "RecordingTime" => "TDRC",
        "ReleaseTime" => "TDRL",
        "TaggingTime" => "TDTG",
        "EncodedBy" => "TENC",
        "InvolvedPeopleList" => "TIPL",
        "ContentGroupDescription" => "TIT1",
        "SubtitleDescriptionRefinement" => "TIT3",
        "InitialKey" => "TKEY",
        "Length" => "TLEN",
        "MusicianCreditsList" => "TMCL",
        "MediaType" => "TMED",
        "Mood" => "TMOO",
        "OriginalAlbumMovieShowTitle" => "TOAL",
        "OriginalLyricistTextWriter" => "TOLY",
        "OriginalArtistPerformer" => "TOPE",
        "FileOwnerLicensee" => "TOWN",
        "ConductorPerformerRefinement" => "TPE3",
        "InterpretedRemixedModifiedBy" => "TPE4",
        "PartOfSet" => "TPOS",
        "ProducedNotice" => "TPRO",
        "Publisher" => "TPUB",
        "InternetRadioStationName" => "TRSN",
        "InternetRadioStationOwner" => "TRSO",
        "AlbumSortOrder" => "TSOA",
        "PerformerSortOrder" => "TSOP",
        "TitleSortOrder" => "TSOT",
        "ISRC" => "TSRC",
        "SoftwareHardwareSettings" => "TSSE",
        "SetSubtitle" => "TSST",
        "UserDefinedTextInformation" => "TXXX",
        "UniqueFileIdentifier" => "UFID",
        "TermsOfUse" => "USER",
        "UnsynchronisedLyricTextTranscription" => "USLT",
        "CommercialInformation" => "WCOM",
        "CopyrightLegalInformation" => "WCOP",
        "OfficialAudioFileWebpage" => "WOAF",
        "OfficialArtistPerformerWebpage" => "WOAR",
        "OfficialAudioSourceWebpage" => "WOAS",
        "OfficialInternetRadioStationHomepage" => "WORS",
        "Payment" => "WPAY",
        "PublishersOfficialWebpage" => "WPUB",
        "UserDefinedURLLink" => "WXXX",
    };
    
    fn get_frame_map() -> &'static Map<&'static str, &'static str> {
        &FRAME_MAP
    }
      
    pub fn get_frame_id(entry: &MetaEntry) -> Option<&'static str> {
        match entry {
            MetaEntry::Custom(_) => None, // Custom entries don't have predefined frame IDs
            _ => {
                let entry_name = format!("{}", entry);
                get_frame_map().get(entry_name.as_str()).copied()
            }
        }
    }
    
    /// Check if a frame ID is supported in ID3v2.3/v2.4
    pub fn is_supported_frame(frame_id: &str) -> bool {
        get_frame_map().values().any(|&id| id == frame_id)
    }
}

/// Frame mapping for ID3v2.0 (3-character frame IDs)
pub mod v2_0 {
    use phf::{phf_map, Map};

    use super::*;
    
    static FRAME_MAP: Map<&'static str, &'static str> = phf_map! {
        "Title" => "TIT",
        "Artist" => "TP1",
        "Album" => "TAL",
        "Date" => "TDA",
        "Genre" => "TCO",
        "TextWriter" => "TXT",
        "AudioEncryption" => "CRA",
        "Language" => "TLA",
        "Time" => "TIM",
        "Composer" => "TCM",
        "FileType" => "TFT",
        "BandOrchestra" => "TP2",
        "RecommendedBufferSize" => "BUF",
        "PlayCounter" => "CNT",
        "Comments" => "COM",
        "EncryptedMetaFrame" => "CRM",
        "EventTimingCodes" => "ETC",
        "Equalization" => "EQU",
        "GeneralEncapsulatedObject" => "GEO",
        "InvolvedPeopleList" => "IPL",
        "LinkedInformation" => "LNK",
        "MusicCDIdentifier" => "MCI",
        "MPEGLocationLookupTable" => "MLL",
        "AttachedPicture" => "PIC",
        "Popularimeter" => "POP",
        "Reverb" => "REV",
        "RelativeVolumeAdjustment" => "RVA",
        "SynchronizedLyricText" => "SLT",
        "SyncedTempoCodes" => "STC",
        "BeatsPerMinute" => "TBP",
        "CopyrightMessage" => "TCR",
        "PlaylistDelay" => "TDY",
        "EncodedBy" => "TEN",
        "InitialKey" => "TKE",
        "Length" => "TLE",
        "MediaType" => "TMT",
        "OriginalArtistPerformer" => "TOA",
        "OriginalFilename" => "TOF",
        "OriginalLyricistTextWriter" => "TOL",
        "OriginalReleaseYear" => "TOR",
        "OriginalAlbumMovieShowTitle" => "TOT",
        "ConductorPerformerRefinement" => "TP3",
        "InterpretedRemixedModifiedBy" => "TP4",
        "PartOfSet" => "TPA",
        "Publisher" => "TPB",
        "ISRC" => "TRC",
        "RecordingDates" => "TRD",
        "TrackNumberPositionInSet" => "TRK",
        "Size" => "TSI",
        "SoftwareHardwareSettings" => "TSS",
        "ContentGroupDescription" => "TT1",
        "TitleSongnameContentDescription" => "TT2",
        "SubtitleDescriptionRefinement" => "TT3",
        "UserDefinedTextInformation" => "TXX",
        "Year" => "TYE",
        "UniqueFileIdentifier" => "UFI",
        "UnsynchronizedLyricTextTranscription" => "ULT",
        "OfficialAudioFileWebpage" => "WAF",
        "OfficialArtistPerformerWebpage" => "WAR",
        "OfficialAudioSourceWebpage" => "WAS",
        "CommercialInformation" => "WCM",
        "CopyrightLegalInformation" => "WCP",
        "PublishersOfficialWebpage" => "WPB",
        "UserDefinedURLLink" => "WXX",
    };
    
    fn get_frame_map() -> &'static Map<&'static str, &'static str> {
        &FRAME_MAP
    }

    pub fn get_frame_id(entry: &MetaEntry) -> Option<&'static str> {
        match entry {
            MetaEntry::Custom(_) => None,
            _ => {
                let entry_name = format!("{}", entry);
                get_frame_map().get(entry_name.as_str()).copied()
            }
        }
    }
    
    /// Check if a frame ID is supported in ID3v2.0
    pub fn is_supported_frame(frame_id: &str) -> bool {
        get_frame_map().values().any(|&id| id == frame_id)
    }
}
