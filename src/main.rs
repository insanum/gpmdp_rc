
extern crate ws;
extern crate serde_json;

use std::env;
use std::num::ParseIntError;

static APP_NAME: &str  = "cli";
static TOKEN: &str     = "be3363fa-0fe3-49de-9c97-3310d66cd3ac";
static URL: &str       = "ws://127.0.0.1:5672";
static REQUEST_ID: u32 = 13;

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
    let value = js.get("value").unwrap();

    for artist in value.get("artists").unwrap().as_array().unwrap() {
        println!("{}", artist["name"].as_str().unwrap());
    }

    for album in value.get("albums").unwrap().as_array().unwrap() {
        println!("{} - {}",
                 album["artist"].as_str().unwrap(),
                 album["name"].as_str().unwrap());
    }

    for track in value.get("tracks").unwrap().as_array().unwrap() {
        println!("{} - {} - {}",
                 track["artist"].as_str().unwrap(),
                 track["album"].as_str().unwrap(),
                 track["title"].as_str().unwrap());
    }
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

pub fn parse_cmd(args: &Vec<String>,
                 cur_track_progress: u64,
                 cur_track_total: u64)
    -> Option<(/* namespace */ String,
               /* method */ String,
               /* arguments */ String,
               /* resp_handler */ fn(serde_json::Value))>
{
    let namespace: &str;
    let method: &str;
    let mut arguments = String::new();
    let mut resp_handler: fn(serde_json::Value) = generic_handler;
    let cmd = args[1].as_str();

    // figure out the command to run and build the command data
    match cmd {
        "help" => {
            usage(args);
            return None;
        }
        "status" => {
            namespace = "playback";
            method = "getPlaybackState";
            resp_handler = get_playback_state_handler;
        }
        "play" => {
            if args.len() == 3 {
                namespace = "queue";
                method = "playTrack";
                match parse_index_num(&args[2]) {
                    Ok(_n) => arguments.push_str(&args[2]),
                    Err(_err) => {
                        println!("ERROR: Failed to parse track number");
                        return None;
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
            if args.len() != 3 {
                println!("ERROR: Must provide seek value");
                return None;
            }
            namespace = "playback";
            method = "setCurrentTime";

            let mut seek_to: i64 = cur_track_progress as i64;
            let seek_amount: i64;

            if args[2].as_str() == "forward" {
                seek_amount = 10000; /* 10s */
            } else if args[2].as_str() == "backward" {
                seek_amount = -10000; /* 10s */
            } else {
                match parse_seek(&args[2]) {
                    Ok(n) => seek_amount = n * 1000, /* convert to ms */
                    Err(_err) => {
                            println!("ERROR: Failed to parse seek value");
                            return None;
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
            if args.len() != 3 {
                println!("ERROR: Must provide thumbs rating");
                return None;
            }
            namespace = "rating";
            if args[2].as_str() == "up" {
                method = "toggleThumbsUp";
            } else if args[2].as_str() == "down" {
                method = "toggleThumbsDown";
            } else {
                println!("ERROR: Invalid thumbs rating");
                return None;
            }
        }
        "shuffle" => {
            if args.len() != 3 {
                println!("ERROR: Must provide shuffle mode");
                return None;
            }
            namespace = "playback";
            method = "setShuffle";
            if args[2].as_str() == "on" {
                arguments.push_str(r#"["ALL_SHUFFLE"]"#);
            } else if args[2].as_str() == "off" {
                arguments.push_str(r#"["NO_SHUFFLE"]"#);
            } else {
                println!("ERROR: Invalid shuffle mode");
                return None;
            }
        }
        "repeat" => {
            if args.len() != 3 {
                println!("ERROR: Must provide repeat mode");
                return None;
            }
            namespace = "playback";
            method = "setRepeat";
            if args[2] == "all" {
                arguments.push_str(r#"["LIST_REPEAT"]"#);
            } else if args[2].as_str() == "single" {
                arguments.push_str(r#"["SINGLE_REPEAT"]"#);
            } else if args[2].as_str() == "off" {
                arguments.push_str(r#"["NO_REPEAT"]"#);
            } else {
                println!("ERROR: Invalid repeat mode");
                return None;
            }
        }
        "clear" => {
            namespace = "queue";
            method = "clear";
        }
        "playlist" => {
            if args.len() != 3 {
                println!("ERROR: Must provide a playlist number");
                return None;
            }
            namespace = "playlists";
            method = "play";
            match parse_index_num(&args[2]) {
                Ok(_n) => arguments.push_str(&args[2]),
                Err(_err) => {
                    println!("ERROR: Failed to parse playlist number");
                    return None;
                }
            }
        }
        "search" => {
            if args.len() != 3 {
                println!("ERROR: Must provide search string");
                return None;
            }
            namespace = "search";
            method = "performSearch";
            resp_handler = perform_search_handler;
            arguments.push_str(&format!(r#"["{}"]"#, args[2]));
        }
        "volume" => {
            namespace = "volume";
            if args.len() == 3 {
                if args[2] == "up" {
                    method = "increaseVolume";
                    arguments.push_str("[10]");
                } else if args[2] == "down" {
                    method = "decreaseVolume";
                    arguments.push_str("[10]");
                } else {
                    method = "setVolume";
                    match parse_volume(&args[2]) {
                        Ok(n) => arguments.push_str(&format!("[{}]", n)),
                        Err(_err) => {
                            println!("ERROR: Failed to parse volume level");
                            return None;
                        }
                    }
                }
            } else {
                method = "getVolume";
                resp_handler = get_volume_handler;
            }
        }
        _ => {
            println!("ERROR: unknown command '{}'", cmd);
            return None;
        }
    }

    return Some((namespace.to_string(),
                 method.to_string(),
                 arguments,
                 resp_handler));
}

struct Client
{
    out: ws::Sender,
    args: Vec<String>,
    resp_handler: fn(serde_json::Value),
    is_status_cmd: bool,
    is_queue_cmd: bool,
    is_playlists_cmd: bool,
    cur_volume: u64,
    cur_shuffle: String,
    cur_repeat: String,
    cur_track_artist: String,
    cur_track_album: String,
    cur_track_title: String,
    cur_track_progress: u64,
    cur_track_total: u64,
    cur_track_liked: bool,
    cur_track_disliked: bool,
    cur_queue: String,
    cur_playlists: String,
    got_all_channels: bool,
    cmd_sent: bool,
}

impl Client
{
    pub fn new(out: ws::Sender, args: &Vec<String>) -> Client
    {
        Client {
            out: out,
            args: args.clone(),
            resp_handler: generic_handler,
            is_status_cmd: args[1].as_str() == "status",
            is_queue_cmd: args[1].as_str() == "queue",
            is_playlists_cmd: args[1].as_str() == "playlists",
            cur_volume: 0,
            cur_shuffle: "".to_string(),
            cur_repeat: "".to_string(),
            cur_track_artist: "".to_string(),
            cur_track_album: "".to_string(),
            cur_track_title: "".to_string(),
            cur_track_progress: 0,
            cur_track_total: 0,
            cur_track_liked: false,
            cur_track_disliked: false,
            cur_queue: "".to_string(),
            cur_playlists: "".to_string(),
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
        let mut auth = String::new();
        auth.push_str(&format!(r#"{{ "{}":"{}", "{}":"{}", "{}":["{}","{}"] }}"#,
                               "namespace", "connect",
                               "method", "connect",
                               "arguments", APP_NAME, TOKEN));
        //println!("{}", auth);

        if let Err(_) = self.out.send(auth) {
            println!("ERROR: failed to send auth message");
        } else {
        }

        return Ok(());
    }

    fn on_error(&mut self, err: ws::Error)
    {
        println!("ERROR: websocket failed ({})", err);
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()>
    {
        let js: serde_json::Value =
            serde_json::from_str(&msg.into_text().unwrap()).unwrap();
        //println!("{:#?}", js);
        if js.get("channel") != None {
            let payload = js.get("payload").unwrap();
            match js.get("channel").unwrap().as_str().unwrap() {
                "volume" => {
                    self.cur_volume = payload.as_u64().unwrap();
                }
                "track" => {
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
                "time" => {
                    self.cur_track_progress =
                        payload.get("current").unwrap().as_u64().unwrap();
                    self.cur_track_total =
                        payload.get("total").unwrap().as_u64().unwrap();
                }
                "rating" => {
                    self.cur_track_liked =
                        payload.get("liked").unwrap().as_bool().unwrap();
                    self.cur_track_disliked =
                        payload.get("disliked").unwrap().as_bool().unwrap();
                }
                "shuffle" => {
                    self.cur_shuffle = payload.as_str().unwrap().to_string();
                }
                "repeat" => {
                    self.cur_repeat = payload.as_str().unwrap().to_string();
                }
                "queue" => {
                    self.cur_queue = payload.to_string();
                }
                "playlists" => {
                    self.cur_playlists = payload.to_string();
                }
                "library" => {
                    /* XXX HACK!
                     * The last channel record seems to always be
                     * the "library". Once we recieve this then we can parse
                     * and send the user command. Probably better to track
                     * which channels we've received and after all have been
                     * seen then send the command. This would remove any
                     * notion of order.
                     */
                    self.got_all_channels = true;
                }
                "API_VERSION" |
                "playState" |
                "search-results" |
                "lyrics" |
                "settings:themeColor" |
                "settings:theme" |
                "settings:themeType" |
                _ => {
                    //println!("{:#?}", js);
                }
            }
        }

        if self.got_all_channels && !self.cmd_sent {
            self.cmd_sent = true;

            if self.is_queue_cmd {

                self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
                get_tracks_handler(&self.cur_queue);

            } else if self.is_playlists_cmd {

                self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
                get_all_playlists_handler(&self.cur_playlists);

            } else {

                let (_n, _m, mut _a, _r) =
                    match parse_cmd(&self.args,
                                    self.cur_track_progress,
                                    self.cur_track_total) {
                        None => {
                            self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
                            return Ok(());
                        },
                        Some(x) => { x },
                    };

                if _n == "queue" && _m == "playTrack" {
                    let track_num = _a.parse::<usize>().unwrap() - 1;
                    let tracks: serde_json::Value =
                        serde_json::from_str(&self.cur_queue).unwrap();
                    _a = format!("[{}]", tracks[track_num].to_string());
                }

                if _n == "playlists" && _m == "play" {
                    let playlist_num = _a.parse::<usize>().unwrap() - 1;
                    let playlists: serde_json::Value =
                        serde_json::from_str(&self.cur_playlists).unwrap();
                    _a = format!("[{}]", playlists[playlist_num].to_string());
                }

                self.resp_handler = _r;
                self.send_cmd(&_n, &_m, &_a)?;

            }
        }

        if self.cmd_sent &&
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

        /* XXX need to handle timeouts... */

        return Ok(());
    }
}

fn usage(args: &Vec<String>)
{
    println!("Usage: {} <command> [ args ]", args[0]);
    println!("  help");
    println!("  status");
    println!("  play [ track# ]");
    println!("  pause");
    println!("  next");
    println!("  prev");
    println!("  replay");
    println!("  seek [ +<secs> | -<secs> | forward | backward ]");
    println!("  thumbs < up | down >");
    println!("  shuffle < on | off >");
    println!("  repeat < all | single | off >");
    println!("  queue");
    println!("  clear");
    println!("  playlists");
    println!("  playlist <playlist#>");
    println!("  search \"<text>\"");
    println!("  volume [ <0-100> | up | down ]");
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);

    if args.len() < 2 {
        println!("ERROR: invalid command line args!");
        usage(&args);
        return;
    }

    // connect to the GPMPD websocket and call the closure
    ws::connect(URL, |out| {
        Client::new(out, &args)
    }).unwrap();
}

