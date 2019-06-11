
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
 *   playlists.play(Playlist)         Playlist object returned from getAll()
 *   playlists.playWithTrack(Playlist, Track)  Track from Playlist data
 *
 * x queue.clear()
 * x queue.getTracks()                Track[]
 *   queue.playTracks()               Track object returned from getTracks()
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

fn send_cmd(out: &ws::Sender,
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
    return out.send(req);
}

fn parse_volume(num_str: &str) -> Result<u32, ParseIntError>
{
    return num_str.parse::<u32>().map(|level| { 
        if level > 100 {
            return 100;
        }
        return level;
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

fn get_tracks_handler(js: serde_json::Value)
{
    let value = js.get("value").unwrap().as_array().unwrap();
    for track in value {
        println!("{} - {}",
                 track["album"].as_str().unwrap(),
                 track["title"].as_str().unwrap());
    }
}

fn get_all_playlists_handler(js: serde_json::Value)
{
    //println!("{:#?}", js);
    let value = js.get("value").unwrap().as_array().unwrap();
    for playlist in value {
        println!("{}", playlist["name"].as_str().unwrap());
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

fn usage(args: Vec<String>)
{
    println!("Usage: {} <command> [ args ]", args[0]);
    println!("  help");
    println!("  status");
    println!("  play");
    println!("  pause");
    println!("  next");
    println!("  prev");
    println!("  thumbs < up | down >");
    println!("  shuffle < on | off >");
    println!("  repeat < all | single | off >");
    println!("  queue");
    println!("  clear");
    println!("  playlists");
    println!("  search \"<text>\"");
    println!("  volume [ <0-100> | up | down ]");
}

struct Client
{
    out: ws::Sender,
    namespace: String,
    method: String,
    arguments: String,
    resp: fn(serde_json::Value),
    cmd: String,
 
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
            send_cmd(&self.out,
                     &self.namespace,
                     &self.method,
                     &self.arguments).unwrap();
        }

        return Ok(());
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        let js: serde_json::Value =
            serde_json::from_str(&msg.into_text().unwrap()).unwrap();
        //println!("{:#?}", js);
        if self.cmd == "status" && js.get("channel") != None {
            match js.get("channel").unwrap().as_str().unwrap() {
                "volume" => {
                    self.cur_volume =
                        js.get("payload").unwrap().as_u64().unwrap();
                }
                "track" => {
                    self.cur_track_artist =
                        js.get("payload").unwrap().get("artist").unwrap().as_str().unwrap().to_string();
                    self.cur_track_album =
                        js.get("payload").unwrap().get("album").unwrap().as_str().unwrap().to_string();
                    self.cur_track_title =
                        js.get("payload").unwrap().get("title").unwrap().as_str().unwrap().to_string();
                }
                "time" => {
                    self.cur_track_progress =
                        js.get("payload").unwrap().get("current").unwrap().as_u64().unwrap();
                    self.cur_track_total =
                        js.get("payload").unwrap().get("total").unwrap().as_u64().unwrap();
                }
                "rating" => {
                    self.cur_track_liked =
                        js.get("payload").unwrap().get("liked").unwrap().as_bool().unwrap();
                    self.cur_track_disliked =
                        js.get("payload").unwrap().get("disliked").unwrap().as_bool().unwrap();
                }
                "shuffle" => {
                    self.cur_shuffle =
                        js.get("payload").unwrap().as_str().unwrap().to_string();
                }
                "repeat" => {
                    self.cur_repeat =
                        js.get("payload").unwrap().as_str().unwrap().to_string();
                }
                "API_VERSION" |
                "playState" |
                "queue" |
                "search-results" |
                "lyrics" |
                "settings:themeColor" |
                "settings:theme" |
                "settings:themeType" |
                "playlists" |
                "library" |
                _ => {
                    //println!("{:#?}", js);
                }
            }
        }

        if js.get("requestID") != None &&
           js.get("requestID").unwrap() == REQUEST_ID {
            self.out.close(ws::CloseCode::Normal).unwrap(); // close connection
            //println!("Got the response!");
            //println!("{:#?}", js);
            (self.resp)(js);

            if self.cmd == "status" {
                println!("artist: {}", self.cur_track_artist);
                println!("album: {}", self.cur_track_album);
                println!("title: {}", self.cur_track_title);
                println!("time: {}/{}",
                         self.cur_track_progress, self.cur_track_total);
                if self.cur_track_liked {
                    println!("rating: up");
                }
                else if self.cur_track_disliked {
                    println!("rating: down");
                }
                println!("volume: {}", self.cur_volume);
                println!("shuffle: {}",
                         if self.cur_shuffle == "NO_SHUFFLE" { "off" }
                         else { "all" });
                println!("repeat: {}",
                         if self.cur_repeat == "LIST_REPEAT" { "all" }
                         else if self.cur_repeat == "SINGLE_REPEAT" { "single" }
                         else { "off" });
            }
        }

        /* XXX need to handle timeouts... */

        return Ok(());
    }
}

fn main()
{
    let args: Vec<String> = env::args().collect();
    //println!("{:#?}", args);

    if args.len() < 2 {
        println!("ERROR: invalid command line args!");
        usage(args);
        return;
    }

    let namespace: &str;
    let method: &str;
    let mut arguments = String::new();
    let resp: fn(serde_json::Value);
    let cmd = args[1].as_str();

    // figure out the command to run and build the request string
    match cmd {
        "help" => {
            usage(args);
            return;
        }
        "status" => {
            namespace = "playback";
            method = "getPlaybackState";
            resp = get_playback_state_handler;
        }
        "play" | "pause" => {
            namespace = "playback";
            method = "playPause";
            resp = generic_handler;
        }
        "next" => {
            namespace = "playback";
            method = "forward";
            resp = generic_handler;
        }
        "prev" => {
            namespace = "playback";
            method = "rewind";
            resp = generic_handler;
        }
        "thumbs" => {
            if args.len() != 3 {
                println!("ERROR: Must provide thumbs rating");
                return;
            }
            namespace = "rating";
            if args[2].as_str() == "up" {
                method = "toggleThumbsUp";
            } else if args[2].as_str() == "down" {
                method = "toggleThumbsDown";
            } else {
                println!("ERROR: Invalid thumbs rating");
                return;
            }
            resp = generic_handler;
        }
        "shuffle" => {
            if args.len() != 3 {
                println!("ERROR: Must provide shuffle mode");
                return;
            }
            namespace = "playback";
            method = "setShuffle";
            if args[2].as_str() == "on" {
                arguments.push_str(r#"["ALL_SHUFFLE"]"#);
            } else if args[2].as_str() == "off" {
                arguments.push_str(r#"["NO_SHUFFLE"]"#);
            } else {
                println!("ERROR: Invalid shuffle mode");
                return;
            }
            resp = generic_handler;
        }
        "repeat" => {
            if args.len() != 3 {
                println!("ERROR: Must provide repeat mode");
                return;
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
                return;
            }
            resp = generic_handler;
        }
        "queue" => {
            namespace = "queue";
            method = "getTracks";
            resp = get_tracks_handler;
        }
        "clear" => {
            namespace = "queue";
            method = "clear";
            resp = generic_handler;
        }
        "playlists" => {
            namespace = "playlists";
            method = "getAll";
            resp = get_all_playlists_handler;
        }
        "search" => {
            if args.len() != 3 {
                println!("ERROR: Must provide search string");
                return;
            }
            namespace = "search";
            method = "performSearch";
            arguments.push_str(&format!("[\"{}\"]", args[2]));
            resp = perform_search_handler;
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
                            println!("ERROR: failed to parse volume level");
                            return;
                        }
                    }
                }
                resp = generic_handler;
            } else {
                method = "getVolume";
                resp = get_volume_handler;
            }
        }
        _ => {
            println!("ERROR: unknown command '{}'", cmd);
            return;
        }
    }

    // connect to the GPMPD websocket and call the closure
    ws::connect(URL, |out| Client { out: out,
                                    namespace: namespace.to_string(),
                                    method: method.to_string(),
                                    arguments: arguments.to_string(),
                                    resp: resp,
                                    cmd: cmd.to_string(),
                                    cur_volume: 0,
                                    cur_shuffle: "".to_string(),
                                    cur_repeat: "".to_string(),
                                    cur_track_artist: "".to_string(),
                                    cur_track_album: "".to_string(),
                                    cur_track_title: "".to_string(),
                                    cur_track_progress: 0,
                                    cur_track_total: 0,
                                    cur_track_liked: false,
                                    cur_track_disliked: false }).unwrap();
}

