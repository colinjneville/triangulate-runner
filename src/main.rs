mod util;

use std::{env, error, fs, io::Write};
use clap::{App, Arg, ArgMatches};
use triangulate::{Triangulate, Vertex, builders};

const PARAM_IN_FILE: &'static str = "in-file";
const PARAM_OUT_FILE: &'static str = "out-file";
const PARAM_DEBUG_DIRECTORY: &'static str = "debug-directory";
const PARAM_DEBUG_NO_LABELS: &'static str = "debug-no-labels";
const PARAM_DEBUG_LEVEL: &'static str = "debug-level";

const ENV_DEBUG_DIRECTORY: &'static str = "TRIANGULATE_SVG_OUTPUT_PATH";
const ENV_DEBUG_NO_LABELS: &'static str = "TRIANGULATE_SVG_HIDE_LABELS";
const ENV_DEBUG_LEVEL: &'static str = "TRIANGULATE_SVG_OUTPUT_LEVEL";

fn main() {
    let matches = App::new("triangulate runner")
        .arg(Arg::with_name(PARAM_IN_FILE)
            .short("i")
            .long(PARAM_IN_FILE)
            .value_name("INFILE")
            .takes_value(true)
            .required(true)
            .index(1)
        )
        .arg(Arg::with_name(PARAM_OUT_FILE)
            .short("o")
            .long(PARAM_OUT_FILE)
            .value_name("OUTFILE")
            .takes_value(true)
        )
        .arg(Arg::with_name(PARAM_DEBUG_DIRECTORY)
            .short("d")
            .long(PARAM_DEBUG_DIRECTORY)
            .value_name("DEBUGDIRECTORY")
            .takes_value(true)
        )
        .arg(Arg::with_name(PARAM_DEBUG_NO_LABELS)
            .long(PARAM_DEBUG_NO_LABELS)
            .value_name("DEBUGNOLABELS")
        )
        .arg(Arg::with_name(PARAM_DEBUG_LEVEL)
            .short("l")
            .long(PARAM_DEBUG_LEVEL)
            .value_name("DEBUGLEVEL")
            .takes_value(true)
        )
        .get_matches();

    if let Err(err) = evaluate(matches) {
        println!("{}", err);
    }
}

fn evaluate(matches: ArgMatches) -> Result<(), Box<dyn error::Error>> {
    let in_file = matches.value_of(PARAM_IN_FILE).unwrap();
    if matches.is_present(PARAM_DEBUG_NO_LABELS) {
        env::set_var(ENV_DEBUG_NO_LABELS, "true");
    }
    if let Some(debug_directory) = matches.value_of(PARAM_DEBUG_DIRECTORY) {
        env::set_var(ENV_DEBUG_DIRECTORY, debug_directory);
    }
    if let Some(debug_level) = matches.value_of(PARAM_DEBUG_LEVEL) {
        env::set_var(ENV_DEBUG_LEVEL, debug_level);
    }

    match util::load_polygon_list(in_file) {
        Ok(t) => {
            match t.triangulate::<builders::VecVecFanBuilder<_>>(&mut Vec::new()) {
                Ok(triangulation) => {
                    if let Some(out_file) = matches.value_of(PARAM_OUT_FILE) {
                        let mut file = fs::File::create(out_file)?;
                        for fan in triangulation {
                            for vertex in fan {
                                writeln!(file, "{} {}", vertex.x(), vertex.y())?;
                            }
                            writeln!(file)?;
                        }
                    }
                    Ok(())
                }
                Err(err) => Err(Box::new(err)),
            }
        }
        Err(err) => Err(err)
    }        
}
