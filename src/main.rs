use chrono::Local;
/// Module to handle generating latex files for ingredients
use std::error::Error;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use std::path::PathBuf;
use structopt::StructOpt;

const DOCUMENT_BEGIN: &str = r#"
\documentclass[12pt]{article}

\usepackage{fullpage}
\usepackage{fontspec}
\usepackage{multicol}

\pagestyle{empty}

\setmainfont{Andika}

\begin{document}
"#;

const HEADING_START: &str = r#"
\begin{center}
{\huge Grocery List}
\medskip

"#;

const HEADING_ENDS: &str = r#"
\end{center}
"#;

const BEGIN_BALANCED_LIST: &str = r#"
\bigskip
\begin{multicols}{2}
\begin{itemize}
{\large
"#;

const END_BALANCED_LIST: &str = r#"
}
\end{itemize}
\end{multicols}
"#;

const DOCUMENT_END: &str = r#"
\end{document}
"#;

/// Command line arguments
#[derive(StructOpt)]
struct Args {
    /// (Input) the file containing our list of URLs
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

/// Ingredients structure
struct Ingredients {
    all: Vec<String>, // All ingredients
}

impl Ingredients {
    pub fn new() -> Ingredients {
        Ingredients { all: Vec::new() }
    }
}

/// Sort our list of ingredients into buckets
///
/// # Arguments
/// * ingredients - Vector of ingredients to be put into our LaTeX document
///
/// # Returns
/// * On success, an Ok() containing an Ingredients structure is returned.
/// * On Failure, an Err() containing (potentially) useful information is returned.
///
fn sort_ingredients(ingredients: Vec<String>) -> Result<Ingredients, Box<dyn Error>> {
    // let mut max: bool = rand::random();
    let mut parsed_ingredients = Ingredients::new();

    for ingredient in ingredients {
        parsed_ingredients.all.push(ingredient.clone());
    }

    Ok(parsed_ingredients)
}

/// Generate a LaTex file for our ingredients
///
/// # Arguments
/// * ingredients - Vector of ingredients to be put into our LaTeX document
///
/// # Returns
/// * On success, an empty Ok() is returned.
/// * On Failure, an Err() containing (potentially) useful information is returned.
///
fn write_ingredients(ingredients: Vec<String>, file: PathBuf) -> Result<(), Box<dyn Error>> {
    let date = Local::now().format("%Y%m%d");
    let sorted_ingredients = sort_ingredients(ingredients)?;

    let mut file = File::create(file)?;

    file.write(DOCUMENT_BEGIN.as_bytes())?;
    file.write(HEADING_START.as_bytes())?;
    file.write(format!("{}\n", date).as_bytes())?;
    file.write(HEADING_ENDS.as_bytes())?;
    file.write(BEGIN_BALANCED_LIST.as_bytes())?;

    for ingredient in sorted_ingredients.all {
        file.write(format!("\\item[] {}\n", ingredient).as_bytes())?;
    }

    file.write(END_BALANCED_LIST.as_bytes())?;

    file.write(DOCUMENT_END.as_bytes())?;

    Ok(())
}

/// Read our file in to a vector of ingredients
///
/// # Arguments
/// * file - Name of file to open and parse
///
/// # Returns
/// * On success, an Ok() containing a Vector ingredients as strings.
/// * On Failure, an Err() containing (potentially) useful information is returned.
///
fn read_file(filename: PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(filename).expect("Could not read input file.");
    let reader = BufReader::new(file);

    let ingredients: Vec<String> = reader
        .lines()
        .map(|line| line.expect("Error reading line"))
        .collect();

    return Ok(ingredients);
}

/// Main function
///
/// # Returns
/// * On success, an empty Ok() is returned.
/// * On Failure, an Err() containing (potentially) useful information is returned.
///
fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::from_args();

    let mut outfile = args.file.clone();
    outfile.set_extension("tex");

    let ingredients = read_file(args.file)?;

    write_ingredients(ingredients, outfile)?;

    Ok(())
}
