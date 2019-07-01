
extern crate getopts;
extern crate ws;
extern crate serde_json;
extern crate yaml_rust;

use std::env;
use std::fs;
use std::io;
use std::io::Write;
use std::num::ParseIntError;
use std::io::{Error, ErrorKind};
use getopts::Options;
use yaml_rust::{Yaml, YamlLoader};

static APP_NAME: &str = "gpmdp_rc";

fn get_config(file: &str) -> Option<std::vec::Vec<Yaml>>
{
    let s = match fs::read_to_string(file) {
        Ok(s) => { s }
        Err(_err) => {
            println!("ERROR: failed to read config file");
            return None;
        }
    };

    match YamlLoader::load_from_str(&s) {
        Ok(yaml) => Some(yaml),
        Err(_err) => {
            println!("ERROR: failed to parse config file");
            return None;
        }
    }
}

/*
 * x volume.getVolume()               0-100
 * x volume.setVolume(num)            0-100
 * x volume.increaseVolume(amount)    default=5
 * x volume.decreaseVolume(amount)    default=5
 *
 *   playback.getCurrentTime()        current track progress in ms
 *   playback.setCurrentTime(ms)      set track progress to ms
 *   playback.getTotalTime()          total track time in ms
 *   playback.isPlaying()             is current track playing
 *   playback.getCurrentTrack()       get current track metadata
 * x playback.playPause()             toggle between play/pause state
 * x playback.getPlaybackState()      0=stopped, 1=paused, 2=playing
 * x playback.forward()               next track
 * x playback.rewind()                previous track
 *   playback.getShuffle()            ALL_SHUFFLE, NO_SHUFFLE
 * x playback.setShuffle(mode)        ALL_SHUFFLE, NO_SHUFFLE
 *   playback.toggleShuffle()         toggle shuffle active/inactive
 *   playback.getRepeat()             LIST_REPEAT, SINGLE_REPEAT, NO_REPEAT
 * x playback.setRepeat(mode)         LIST_REPEAT, SINGLE_REPEAT, NO_REPEAT
 *   playback.toggleRepeat()          NO_REPEAT -> LIST_REPEAST -> SINGLE_REPEAT
 *   playback.isPodcast()             is current track a podcast
 *   playback.forwardThirty()         podcasts only
 *   playback.rewindTen()             podcasts only
 *
 *   rating.getRating()               current rack rating (0-5)
 *                                    0 = no rating, 1 = thumbs down, 5 = thumbs up
 * x rating.toggleThumbsUp()          if Down then it's removed, else toggle Up
 * x rating.toggleThumbsDown()        if Up then it's removed, else toggle Down
 *   rating.setRating(rating)         set current track rating (1-5)
 *   rating.resetRating()             removes current track rating
 *
 * x playlists.getAll()               Playlist[]
 * x playlists.play(Playlist)         Playlist object returned from getAll()
 *   playlists.playWithTrack(Playlist, Track)  Track from Playlist data
 *
 * x queue.clear()
 * x queue.getTracks()                Track[]
 * x queue.playTrack(Track)           Track object returned from getTracks()
 *
 *   search.getCurrentResults()
 *   search.getSearchText()
 *   search.isSearching()
 * x search.performSearch(text)
 *   search.playResult(result)        Artist,Album,Track from getCurrentResults()
 *
 * Album {
 *   "id": String,           // Unique ID for this album
 *   "name": String,         // The name of the album
 *   "artist": String,       // The name of the artist for the album
 *   "albumArt": String,     // URL to the albumArt for this album
 *   "tracks": Track[]
 * }
 *
 * Artist {
 *   "id": String,           // Unique ID for this artist
 *   "name": String,         // The name of the artist
 *   "image": String,        // URL to an image of this artist
 *   "albums": Album[]
 * }
 *
 * Playlist {
 *   "id": String,           // Unique ID for this playlist
 *   "name": String,         // User defined name for this playlist
 *   "tracks": Track[],      // An array of Track objects that make up the playlist
 * }
 *
 * SearchResults {
 *   "searchText": String,   // search text used to get these results
 *   "bestMatch": {
 *     "type": String,       // Best results, one of Album, Artist, or Track
 *     "value": Album | Artist | Track
 *   }
 *   "albums": Album[],      // An array of albums
 *   "artists": Artist[],    // An array of artists
 *   "tracks": Track[],      // An array of tracks
 * }
 *
 * Track {
 *   "id":
 *   "title":
 *   "albumArt":
 *   "artist":
 *   "album":
 *   "albumArtist":
 *   "index":
 *   "duration":
 *   "playCount":
 *   "albumId":
 *   "artistId":
 *   "artistImage":
 * }
 */

fn parse_volume(num_str: &str) -> Result<u32, ParseIntError>
{
    return num_str.parse::<u32>().map(|level| {
        if level > 100 {
            return 100;
        }
        return level;
    });
}

fn parse_seek(seek_str: &str) -> Result<i64, ParseIntError>
{
    return seek_str.parse::<i64>().map(|seek| {
        return seek;
    });
}

fn parse_index_num(index_num_str: &str) -> Result<u32, ParseIntError>
{
    return index_num_str.parse::<u32>().map(|track| {
        return track;
    });
}

fn auth_handler() -> String
{
    let mut code = String::new();
    print!("Enter the 4-digit code from GPMDP: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut code).expect("invalid code");
    if let Some('\n') = code.chars().next_back() { code.pop(); }
    if let Some('\r') = code.chars().next_back() { code.pop(); }
    return code;
}

fn get_playback_state_handler(js: serde_json::Value)
{
    let value = js.get("value").unwrap().as_u64().unwrap();
    match value {
        0 => println!("state: stopped"),
        1 => println!("state: paused"),
        2 => println!("state: playing"),
        _ => println!("state: unknown"),
    }
}

fn get_tracks_handler(js: &str)
{
    let tracks: serde_json::Value = serde_json::from_str(&js).unwrap();
    for i in 0..tracks.as_array().unwrap().len() {
        println!("{}: {} | {} | {}", (i + 1),
                 tracks[i]["artist"].as_str().unwrap(),
                 tracks[i]["album"].as_str().unwrap(),
                 tracks[i]["title"].as_str().unwrap());
    }
}

fn lyrics_handler(lyrics: &str)
{
    if lyrics.is_empty() {
        println!("Lyrics not available!");
    } else {
        println!("{}", lyrics);
    }
}

fn get_all_playlists_handler(js: &str)
{
    let playlists: serde_json::Value = serde_json::from_str(&js).unwrap();
    for i in 0..playlists.as_array().unwrap().len() {
        println!("{}: {}", (i + 1),
                 playlists[i]["name"].as_str().unwrap());
    }
}

fn perform_search_handler(js: serde_json::Value)
{
    //println!("{:#?}", js);
    let value: &serde_json::Value;
    if js.get("value") == None {
        value = &js;
    } else {
        value = js.get("value").unwrap();
    }

    let mut j = 1;

    let artists = value.get("artists").unwrap().as_array().unwrap();
    if artists.len() > 0 {
        for i in 0..artists.len() {
            println!("{}: {}", (i + j),
                     artists[i]["name"].as_str().unwrap());
        }
        j += artists.len();
    }

    let albums = value.get("albums").unwrap().as_array().unwrap();
    if albums.len() > 0 {
        for i in 0..albums.len() {
            println!("{}: {} | {}", (i + j),
                     albums[i]["artist"].as_str().unwrap(),
                     albums[i]["name"].as_str().unwrap());
        }
        j += albums.len();
    }

    let tracks = value.get("tracks").unwrap().as_array().unwrap();
    if tracks.len() > 0 {
        for i in 0..tracks.len() {
            println!("{}: {} | {} | {}", (i + j),
                     tracks[i]["artist"].as_str().unwrap(),
                     tracks[i]["album"].as_str().unwrap(),
                     tracks[i]["title"].as_str().unwrap());
        }
    }
}

fn list_search_results_handler(js: &str)
{
    let value: serde_json::Value = serde_json::from_str(&js).unwrap();
    perform_search_handler(value);
}

fn get_volume_handler(js: serde_json::Value)
{
    let value = js.get("value").unwrap().as_u64().unwrap();
    println!("{}", value);
}

fn generic_handler(_js: serde_json::Value)
{
    //println!("{:#?}", _js);
}

fn parse_cmd(cmd: &Vec<String>,
             cur_track_progress: u64,
             cur_track_total: u64)
    -> Result<(/* namespace */ String,
               /* method */ String,
               /* arguments */ String,
               /* resp_handler */ fn(serde_json::Value)),
              String>
{
    let namespace: &str;
    let method: &str;
    let mut arguments = String::new();
    let mut resp_handler: fn(serde_json::Value) = generic_handler;

    // figure out the command to run and build the command data
    match cmd[0].as_str() {
        "status" => {
            namespace = "playback";
            method = "getPlaybackState";
            resp_handler = get_playback_state_handler;
        }
        "play" => {
            if cmd.len() == 2 {
                namespace = "queue";
                method = "playTrack";
                match parse_index_num(&cmd[1]) {
                    Ok(_n) => arguments.push_str(&cmd[1]),
                    Err(_err) => {
                        return Err("failed to parse track number".to_string());
                    }
                }
            } else {
                namespace = "playback";
                method = "playPause";
            }
        }
        "pause" => {
            namespace = "playback";
            method = "playPause";
        }
        "next" => {
            namespace = "playback";
            method = "forward";
        }
        "prev" => {
            namespace = "playback";
            method = "rewind";
        }
        "replay" => {
            namespace = "playback";
            method = "setCurrentTime";
            arguments.push_str("[0]");
        }
        "seek" => {
            if cmd.len() != 2 {
                return Err("must provide seek value".to_string());
            }
            namespace = "playback";
            method = "setCurrentTime";

            let mut seek_to: i64 = cur_track_progress as i64;
            let seek_amount: i64;

            if cmd[1].as_str() == "forward" {
                seek_amount = 10000; /* 10s */
            } else if cmd[1].as_str() == "backward" {
                seek_amount = -10000; /* 10s */
            } else {
                match parse_seek(&cmd[1]) {
                    Ok(n) => seek_amount = n * 1000, /* convert to ms */
                    Err(_err) => {
                        return Err("failed to parse seek value".to_string());
                    }
                }
            }

            seek_to += seek_amount; /* up or down */
            if seek_to > cur_track_total as i64 {
                seek_to = cur_track_total as i64;
            } else if seek_to < 0 {
                seek_to = 0;
            }

            arguments.push_str(&format!(r#"[{}]"#, seek_to));
        }
        "thumbs" => {
            if cmd.len() != 2 {
                return Err("must provide thumbs rating".to_string());
            }
            namespace = "rating";
            if cmd[1].as_str() == "up" {
                method = "toggleThumbsUp";
            } else if cmd[1].as_str() == "down" {
                method = "toggleThumbsDown";
            } else {
                return Err("invalid thumbs rating".to_string());
            }
        }
        "shuffle" => {
            if cmd.len() != 2 {
                return Err("must provide shuffle mode".to_string());
            }
            namespace = "playback";
            method = "setShuffle";
            if cmd[1].as_str() == "on" {
                arguments.push_str(r#"["ALL_SHUFFLE"]"#);
            } else if cmd[1].as_str() == "off" {
                arguments.push_str(r#"["NO_SHUFFLE"]"#);
            } else {
                return Err("invalid shuffle mode".to_string());
            }
        }
        "repeat" => {
            if cmd.len() != 2 {
                return Err("must provide repeat mode".to_string());
            }
            namespace = "playback";
            method = "setRepeat";
            if cmd[1] == "all" {
                arguments.push_str(r#"["LIST_REPEAT"]"#);
            } else if cmd[1].as_str() == "single" {
                arguments.push_str(r#"["SINGLE_REPEAT"]"#);
            } else if cmd[1].as_str() == "off" {
                arguments.push_str(r#"["NO_REPEAT"]"#);
            } else {
                return Err("invalid repeat mode".to_string());
            }
        }
        "clear" => {
            namespace = "queue";
            method = "clear";
        }
        "playlist" => {
            if cmd.len() != 2 {
                return Err("must provide a playlist number".to_string());
            }
            namespace = "playlists";
            method = "play";
            match parse_index_num(&cmd[1]) {
                Ok(_n) => arguments.push_str(&cmd[1]),
                Err(_err) => {
                    return Err("failed to parse playlist number".to_string());
                }
            }
        }
        "search" => {
            if cmd.len() != 2 {
                return Err("must provide search string".to_string());
            }
            namespace = "search";
            method = "performSearch";
            resp_handler = perform_search_handler;
            arguments.push_str(&format!(r#"["{}"]"#, cmd[1]));
        }
        "results" => {
            if cmd.len() != 2 {
                return Err("must provide result number".to_string());
            }
            namespace = "search";
            method = "playResult";
            match parse_index_num(&cmd[1]) {
                Ok(_n) => arguments.push_str(&cmd[1]),
                Err(_err) => {
                    return Err("failed to parse result number".to_string());
                }
            }
        }
        "volume" => {
            namespace = "volume";
            if cmd.len() == 2 {
                if cmd[1] == "up" {
                    method = "increaseVolume";
                    arguments.push_str("[10]");
                } else if cmd[1] == "down" {
                    method = "decreaseVolume";
                    arguments.push_str("[10]");
                } else {
                    method = "setVolume";
                    match parse_volume(&cmd[1]) {
                        Ok(n) => arguments.push_str(&format!("[{}]", n)),
                        Err(_err) => {
                            return Err("failed to parse volume level".to_string());
                        }
                    }
                }
            } else {
                method = "getVolume";
                resp_handler = get_volume_handler;
            }
        }
        _ => {
            return Err(format!("unknown command '{}'", cmd[0]));
        }
    }

    return Ok((namespace.to_string(),
               method.to_string(),
               arguments,
               resp_handler));
}

const CHNL_API_VERSION: u64         = 0x0001;
const CHNL_PLAYSTATE: u64           = 0x0002;
const CHNL_TRACK: u64               = 0x0004;
const CHNL_LYRICS: u64              = 0x0008;
const CHNL_TIME: u64                = 0x0010;
const CHNL_RATING: u64              = 0x0020;
const CHNL_SHUFFLE: u64             = 0x0040;
const CHNL_REPEAT: u64              = 0x0080;
const CHNL_PLAYLISTS: u64           = 0x0100;
const CHNL_QUEUE: u64               = 0x0200;
const CHNL_SEARCH_RESULTS: u64      = 0x0400;
const CHNL_LIBRARY: u64             = 0x0800;
const CHNL_VOLUME: u64              = 0x1000;
const CHNL_SETTINGS_THEMECOLOR: u64 = 0x2000;
const CHNL_SETTINGS_THEME: u64      = 0x4000;
const CHNL_SETTINGS_THEMETYPE: u64  = 0x8000;

const CHNLS_ALL: u64           = 0x17FE; //0xFFFF;
const CHNLS_FOR_STATUS: u64    = (CHNL_PLAYSTATE |
                                  CHNL_TRACK |
                                  CHNL_TIME |
                                  CHNL_RATING |
                                  CHNL_SHUFFLE |
                                  CHNL_REPEAT |
                                  CHNL_QUEUE |
                                  CHNL_VOLUME);
const CHNLS_FOR_AUTH: u64      = 0;
const CHNLS_FOR_PLAY: u64      = CHNL_QUEUE;
const CHNLS_FOR_PAUSE: u64     = 0;
const CHNLS_FOR_NEXT: u64      = 0;
const CHNLS_FOR_PREV: u64      = 0;
const CHNLS_FOR_REPLAY: u64    = 0;
const CHNLS_FOR_SEEK: u64      = CHNL_TIME;
const CHNLS_FOR_LYRICS: u64    = CHNL_LYRICS;
const CHNLS_FOR_THUMBS: u64    = 0;
const CHNLS_FOR_SHUFFLE: u64   = 0;
const CHNLS_FOR_REPEAT: u64    = 0;
const CHNLS_FOR_QUEUE: u64     = CHNL_QUEUE;
const CHNLS_FOR_CLEAR: u64     = 0;
const CHNLS_FOR_PLAYLISTS: u64 = CHNL_PLAYLISTS;
const CHNLS_FOR_PLAYLIST: u64  = CHNL_PLAYLISTS;
const CHNLS_FOR_SEARCH: u64    = 0;
const CHNLS_FOR_RESULTS: u64   = CHNL_SEARCH_RESULTS;
const CHNLS_FOR_VOLUME: u64    = 0;

const TIMEOUT_EVENT: ws::util::Token = ws::util::Token(1);
const TIMEOUT_MSECS: u64             = 4000; // 4secs

const REQUEST_ID: u32 = 13;

struct Client
{
    out: ws::Sender,
    cmd: Vec<String>,
    token: String,
    rcvd_new_auth_token: bool,
    resp_handler: fn(serde_json::Value),
    is_status_cmd: bool,
    is_lyrics_cmd: bool,
    is_queue_cmd: bool,
    is_playlists_cmd: bool,
    is_result_no_txt_cmd: bool,
    cur_volume: u64,
    cur_shuffle: String,
    cur_repeat: String,
    cur_track_artist: String,
    cur_track_album: String,
    cur_track_title: String,
    cur_track_lyrics: String,
    cur_track_progress: u64,
    cur_track_total: u64,
    cur_track_liked: bool,
    cur_track_disliked: bool,
    cur_queue: String,
    cur_playlists: String,
    cur_search: String,
    chnls_to_wait_for: u64,
    chnls_rcvd: u64,
    got_all_channels: bool,
    cmd_sent: bool,
}

impl Client
{
    pub fn new(out: ws::Sender,
               cmd: &Vec<String>,
               token: &str) -> Client
    {
        Client {
            out: out,
            cmd: cmd.clone(),
            token: token.to_string(),
            rcvd_new_auth_token: false,
            resp_handler: generic_handler,
            is_status_cmd: cmd[0].as_str() == "status",
            is_lyrics_cmd: cmd[0].as_str() == "lyrics",
            is_queue_cmd: cmd[0].as_str() == "queue",
            is_playlists_cmd: cmd[0].as_str() == "playlists",
            is_result_no_txt_cmd:
                (cmd[0].as_str() == "results") && (cmd.len() == 1),
            cur_volume: 0,
            cur_shuffle: "".to_string(),
            cur_repeat: "".to_string(),
            cur_track_artist: "".to_string(),
            cur_track_album: "".to_string(),
            cur_track_title: "".to_string(),
            cur_track_lyrics: "".to_string(),
            cur_track_progress: 0,
            cur_track_total: 0,
            cur_track_liked: false,
            cur_track_disliked: false,
            cur_queue: "".to_string(),
            cur_playlists: "".to_string(),
            cur_search: "".to_string(),
            chnls_to_wait_for: 0,
            chnls_rcvd: 0,
            got_all_channels: false,
            cmd_sent: false,
        }
    }

    pub fn send_cmd(&mut self,
                    namespace: &str,
                    method: &str,
                    arguments: &str) -> Result<(), ws::Error>
    {
        let mut req = String::new();
        if arguments.is_empty() {
            req.push_str(&format!(r#"{{ "{}":"{}", "{}":"{}", "{}":{} }}"#,
                                  "namespace", namespace,
                                  "method", method,
                                  "requestID", REQUEST_ID.to_string()));
        } else {
            req.push_str(&format!(r#"{{ "{}":"{}", "{}":"{}", "{}":{}, "{}":{} }}"#,
                                  "namespace", namespace,
                                  "method", method,
                                  "requestID", REQUEST_ID.to_string(),
                                  "arguments", arguments));
        }

        //println!("{}", req);
        return self.out.send(req);
    }
}

impl ws::Handler for Client
{
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()>
    {
        self.chnls_to_wait_for = match self.cmd[0].as_str() {
            "status"    => CHNLS_FOR_STATUS,
            "auth"      => CHNLS_FOR_AUTH,
            "play"      => CHNLS_FOR_PLAY,
            "pause"     => CHNLS_FOR_PAUSE,
            "next"      => CHNLS_FOR_NEXT,
            "prev"      => CHNLS_FOR_PREV,
            "replay"    => CHNLS_FOR_REPLAY,
            "seek"      => CHNLS_FOR_SEEK,
            "lyrics"    => CHNLS_FOR_LYRICS,
            "thumbs"    => CHNLS_FOR_THUMBS,
            "shuffle"   => CHNLS_FOR_SHUFFLE,
            "repeat"    => CHNLS_FOR_REPEAT,
            "queue"     => CHNLS_FOR_QUEUE,
            "clear"     => CHNLS_FOR_CLEAR,
            "playlists" => CHNLS_FOR_PLAYLISTS,
            "playlist"  => CHNLS_FOR_PLAYLIST,
            "search"    => CHNLS_FOR_SEARCH,
            "results"   => CHNLS_FOR_RESULTS,
            "volume"    => CHNLS_FOR_VOLUME,
            _           => CHNLS_ALL
        };

        if self.cmd[0] == "auth" {
            self.cmd_sent = true;
            return self.send_cmd("connect", "connect",
                                 &format!(r#"["{}"]"#, APP_NAME));
        }

        let mut auth = String::new();
        auth.push_str(&format!(r#"{{ "{}":"{}", "{}":"{}", "{}":["{}","{}"] }}"#,
                               "namespace", "connect",
                               "method", "connect",
                               "arguments", APP_NAME, self.token));

        if let Err(_) = self.out.send(auth) {
            return Err(ws::Error::from(Error::new(ErrorKind::Other, "failed to send auth message")));
        }

        self.out.timeout(TIMEOUT_MSECS, TIMEOUT_EVENT)?;

        return Ok(());
    }

    fn on_error(&mut self, err: ws::Error)
    {
        println!("ERROR: {}", err);
    }

    fn on_timeout(&mut self, event: ws::util::Token) -> ws::Result<()>
    {
        if event == TIMEOUT_EVENT {
            return Err(ws::Error::from(Error::new(ErrorKind::Other, "command timeout")));
        }

        return Ok(());
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()>
    {
        let js: serde_json::Value =
            serde_json::from_str(&msg.into_text().unwrap()).unwrap();
        //println!("{:#?}", js);
        if js.get("channel") != None {
            let payload = js.get("payload").unwrap();
            match js.get("channel").unwrap().as_str().unwrap() {
                "connect" => {
                    if self.cmd[0] == "auth2" &&
                       payload.as_str().unwrap() != "CODE_REQUIRED" {
                        self.token = payload.as_str().unwrap().to_string();
                        self.rcvd_new_auth_token = true;
                    }
                }
                "API_VERSION" => {
                    self.chnls_rcvd |= CHNL_API_VERSION;
                }
                "playState" => {
                    self.chnls_rcvd |= CHNL_PLAYSTATE;
                }
                "track" => {
                    self.chnls_rcvd |= CHNL_TRACK;
                    if !payload.get("artist").unwrap().is_null() {
                        self.cur_track_artist =
                            payload.get("artist").unwrap().as_str().unwrap().to_string();
                    }
                    if !payload.get("album").unwrap().is_null() {
                        self.cur_track_album =
                            payload.get("album").unwrap().as_str().unwrap().to_string();
                    }
                    if !payload.get("title").unwrap().is_null() {
                        self.cur_track_title =
                            payload.get("title").unwrap().as_str().unwrap().to_string();
                    }
                }
                "lyrics" => {
                    self.chnls_rcvd |= CHNL_LYRICS;
                    if !payload.is_null() {
                        self.cur_track_lyrics = payload.as_str().unwrap().to_string();
                    }
                }
                "time" => {
                    self.chnls_rcvd |= CHNL_TIME;
                    self.cur_track_progress =
                        payload.get("current").unwrap().as_u64().unwrap();
                    self.cur_track_total =
                        payload.get("total").unwrap().as_u64().unwrap();
                }
                "rating" => {
                    self.chnls_rcvd |= CHNL_RATING;
                    self.cur_track_liked =
                        payload.get("liked").unwrap().as_bool().unwrap();
                    self.cur_track_disliked =
                        payload.get("disliked").unwrap().as_bool().unwrap();
                }
                "shuffle" => {
                    self.chnls_rcvd |= CHNL_SHUFFLE;
                    self.cur_shuffle = payload.as_str().unwrap().to_string();
                }
                "repeat" => {
                    self.chnls_rcvd |= CHNL_REPEAT;
                    self.cur_repeat = payload.as_str().unwrap().to_string();
                }
                "playlists" => {
                    self.chnls_rcvd |= CHNL_PLAYLISTS;
                    self.cur_playlists = payload.to_string();
                }
                "queue" => {
                    self.chnls_rcvd |= CHNL_QUEUE;
                    self.cur_queue = payload.to_string();
                }
                "search-results" => {
                    self.chnls_rcvd |= CHNL_SEARCH_RESULTS;
                    self.cur_search = payload.to_string();
                }
                "library" => {
                    self.chnls_rcvd |= CHNL_LIBRARY;
                }
                "volume" => {
                    self.chnls_rcvd |= CHNL_VOLUME;
                    self.cur_volume = payload.as_u64().unwrap();
                }
                "settings:themeColor" => {
                    self.chnls_rcvd |= CHNL_SETTINGS_THEMECOLOR;
                }
                "settings:theme" => {
                    self.chnls_rcvd |= CHNL_SETTINGS_THEME;
                }
                "settings:themeType" => {
                    self.chnls_rcvd |= CHNL_SETTINGS_THEMETYPE;
                }
                _ => {
                    //println!("{:#?}", js);
                }
            }

            if (self.chnls_rcvd & self.chnls_to_wait_for) == self.chnls_to_wait_for {
                self.got_all_channels = true;
            }
        }

        if self.got_all_channels && !self.cmd_sent {
            self.cmd_sent = true;

            if self.is_queue_cmd {

                self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
                get_tracks_handler(&self.cur_queue);

            } else if self.is_lyrics_cmd {

                self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
                lyrics_handler(&self.cur_track_lyrics);

            } else if self.is_playlists_cmd {

                self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
                get_all_playlists_handler(&self.cur_playlists);

            } else if self.is_result_no_txt_cmd {

                self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
                list_search_results_handler(&self.cur_search);

            } else {

                let (_n, _m, mut _a, _r) =
                    match parse_cmd(&self.cmd,
                                    self.cur_track_progress,
                                    self.cur_track_total) {
                        Err(e) => {
                            self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
                            return Err(ws::Error::from(Error::new(ErrorKind::Other, e)));
                        },
                        Ok(x) => { x }
                    };

                if _n == "queue" && _m == "playTrack" {
                    let track_num = _a.parse::<usize>().unwrap() - 1;
                    let tracks: serde_json::Value =
                        serde_json::from_str(&self.cur_queue).unwrap();
                    _a = format!("[{}]", tracks[track_num].to_string());
                }
                else if _n == "playlists" && _m == "play" {
                    let playlist_num = _a.parse::<usize>().unwrap() - 1;
                    let playlists: serde_json::Value =
                        serde_json::from_str(&self.cur_playlists).unwrap();
                    _a = format!("[{}]", playlists[playlist_num].to_string());
                }
                else if _n == "search" && _m == "playResult" {
                    let result_num = _a.parse::<usize>().unwrap() - 1;
                    let results: serde_json::Value =
                        serde_json::from_str(&self.cur_search).unwrap();
                    let artists = results.get("artists").unwrap().as_array().unwrap();
                    let albums = results.get("albums").unwrap().as_array().unwrap();
                    let tracks = results.get("tracks").unwrap().as_array().unwrap();
                    if result_num < artists.len() {
                        _a = format!("[{}]", artists[result_num].to_string());
                    } else if result_num < (artists.len() + albums.len()) {
                        _a = format!("[{}]", albums[result_num - artists.len()].to_string());
                    } else if result_num < (artists.len() + albums.len() + tracks.len()) {
                        _a = format!("[{}]", tracks[result_num - artists.len() - albums.len()].to_string());
                    } else {
                        self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
                        //println!("ERROR: invalid result number");
                        return Err(ws::Error::from(Error::new(ErrorKind::Other, "invalid result number")));
                    }
                }

                self.resp_handler = _r;
                return self.send_cmd(&_n, &_m, &_a);

            }
        }

        if self.got_all_channels && self.cmd_sent && self.cmd[0] == "auth" {
            let code = auth_handler();
            self.cmd[0] = "auth2".to_string();
            return self.send_cmd("connect", "connect",
                                 &format!(r#"["{}", "{}"]"#, APP_NAME, code));
        }
        else if self.got_all_channels && self.cmd_sent &&
                self.cmd[0] == "auth2" && self.rcvd_new_auth_token {
            self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
            println!("Token: {}", self.token);
        }
        else if self.cmd_sent &&
                js.get("requestID") != None &&
                js.get("requestID").unwrap() == REQUEST_ID {
            self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
            //println!("Got the response!");
            //println!("{:#?}", js);
            (self.resp_handler)(js);

            let fmt_time = |ms| {
                let t = ms / 1000; /* convert ms to secs */
                let h = 3600; /* secs in an hour */
                let m = 60; /* secs in a minute */
                if t >= h { /* >= 1 hour */
                    format!("{}:{}:{:02}", (t / h), ((t % h) / m), ((t % h) % m))
                } else if t >= m { /* >= 1 minute */
                    format!("{}:{:02}", (t / m), (t % m))
                } else { /* < 1 minute */
                    format!("0:{:02}", t)
                }
            };

            if self.is_status_cmd {
                println!("artist: {}", self.cur_track_artist);
                println!("album: {}", self.cur_track_album);
                println!("title: {}", self.cur_track_title);
                println!("time_elapsed_fmt: {}", fmt_time(self.cur_track_progress));
                println!("time_elapsed_secs: {}", (self.cur_track_progress / 1000));
                println!("time_total_fmt: {}", fmt_time(self.cur_track_total));
                println!("time_total_secs: {}", (self.cur_track_total / 1000));
                println!("rating: {}",
                         if self.cur_track_liked { "up" }
                         else if self.cur_track_disliked { "down" }
                         else { "none" });
                println!("volume: {}", self.cur_volume);
                println!("shuffle: {}",
                         if self.cur_shuffle == "NO_SHUFFLE" { "off" }
                         else { "on" });
                println!("repeat: {}",
                         if self.cur_repeat == "LIST_REPEAT" { "all" }
                         else if self.cur_repeat == "SINGLE_REPEAT" { "single" }
                         else { "off" });
                let tracks: serde_json::Value =
                    serde_json::from_str(&self.cur_queue).unwrap();
                let mut idx = 0;
                for i in 0..tracks.as_array().unwrap().len() {
                    if tracks[i]["artist"].as_str().unwrap() == self.cur_track_artist &&
                       tracks[i]["album"].as_str().unwrap() == self.cur_track_album &&
                       tracks[i]["title"].as_str().unwrap() == self.cur_track_title {
                            idx = i + 1;
                            break;
                    }
                }
                println!("queue_track: {}", idx);
                println!("queue_length: {}", tracks.as_array().unwrap().len());
            }
        }

        return Ok(());
    }
}

fn usage(cmd: &str)
{
    println!("Usage: {} <command> [ args ]", cmd);
    println!("  auth");
    println!("  status");
    println!("  play [ <track#> ]");
    println!("  pause");
    println!("  next");
    println!("  prev");
    println!("  replay");
    println!("  seek [ +<secs> | -<secs> | forward | backward ]");
    println!("  lyrics");
    println!("  thumbs < up | down >");
    println!("  shuffle < on | off >");
    println!("  repeat < all | single | off >");
    println!("  queue");
    println!("  clear");
    println!("  playlists");
    println!("  playlist <playlist#>");
    println!("  search \"<text>\"");
    println!("  results [ <result#> ]");
    println!("  volume [ <0-100> | up | down ]");
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);

    let mut opts = Options::new();
    opts.optflag("h", "help", "print usage");
    opts.optopt("c", "config", "config file", "CONFIG");
    let options = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if options.opt_present("h") {
        usage(&args[0]);
        std::process::exit(0);
    }

    let config_file = match options.opt_str("c") {
        Some(cf) => { cf }
        None => { format!("{}/gpmdp_rc.yaml", env::var("HOME").unwrap()) }
    };

    if options.free.is_empty() {
        println!("ERROR: invalid command line args!");
        usage(&args[0]);
        std::process::exit(1);
    }

    let cmd = options.free;

    let token: String;
    let url: String;

    match get_config(&config_file) {
        Some(cfg) => {
            token = cfg[0]["token"].as_str().unwrap().to_string();
            url   = cfg[0]["url"].as_str().unwrap().to_string();
        }
        None => {
            std::process::exit(1);
        }
    }

    // connect to the GPMPD websocket and call the closure
    match ws::connect(url, |out| {
        Client::new(out, &cmd, &token)
    }) {
        Ok(_)  => std::process::exit(0),
        Err(_) => std::process::exit(1)
    }
}

