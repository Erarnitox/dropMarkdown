use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_args(&args);
    
    let out_filename  =  &filename[..(filename.len()-2)];
    let out_pdf = String::from(out_filename) + "pdf";
    let out_filename = String::from(out_filename) + "ms";

    let ms_string = create_ms(&filename);
    write_ms(&ms_string, &out_filename);

    create_pdf(&out_filename, &out_pdf);
}

fn create_pdf(ms_file: &String, pdf_file: &String){
    let output = Command::new("groff")
        .arg("-mspdf")
        .arg("-Tpdf")
        .arg("-s")
        .arg("-e")
        .arg("-p")
        .arg("-t")
        .arg("-R")
        .arg(ms_file)
        .output()
        .expect("Failed to call groff. Make sure groff is installed!");

        let mut out_pdf = std::fs::File::create(&pdf_file).expect("Can't create the pdf file!");
        out_pdf.write_all(output.stdout.as_slice()).expect("Can't write to pdf file!");
}

fn write_ms(ms_string : &String, ms_file : &String){
    let mut out_file = std::fs::File::create(&ms_file).expect("Can't create the ms file! ;(");
    out_file.write_all(ms_string.as_bytes()).expect("Can't write to ms file :/");
}

fn parse_args(args: &[String]) -> &String {
    &args[1]
}

fn read_contents(filename: &String) -> String {
    fs::read_to_string(filename)
        .expect("Can't open the file!")
}

fn create_ms(drop_file: &String) -> String {
    let contents: String = read_contents(&drop_file);
    let mut ms_string = String::new();

    ms_string += ".R1\naccumulate\n\ndatabase bib.ref\n\nmove-punctuation\n\n.R2\n\n";

    let mut in_paragraph: bool = false;
    let mut in_quote: bool = false;
    for line in contents.lines(){
        if line.starts_with("#T "){
            ms_string += ".TL\n";
            ms_string += &line[3..];
        }else if line.starts_with("#A "){
            ms_string += "\n.AU\n";
            ms_string += &line[3..];
        }else if line.starts_with("#I "){
            ms_string += "\n.AI\n";
            ms_string += &line[3..];
        }else if line.starts_with("#Date"){
            ms_string += ".DA\n";
        }else if line.starts_with("#Break"){
            ms_string += ".bp\n";
        }else if line.starts_with("#SmallerText"){
            ms_string += ".SM\n";
        }else if line.starts_with("#LargerText"){
            ms_string += ".LG\n";
        }else if line.starts_with("#NormalText"){
            ms_string += ".NL\n";
        }else if line.starts_with("#TOC"){
            ms_string += ".TC\n";
        }else if line.starts_with("#AbstractBegin"){
            ms_string += ".AB\n";
        }else if line.starts_with("#AbstractEnd"){
            ms_string += "\n.AE\n";
        }else if line.starts_with("# "){
            ms_string += ".NH 1\n";
            ms_string += &line[2..];
            ms_string += "\n.XS\n";
            ms_string += &line[2..];
            ms_string += "\n.XE\n";
        }else if line.starts_with("## "){
            ms_string += ".NH 2\n";
            ms_string += &line[3..];
            ms_string += "\n.XS\n";
            ms_string += &line[3..];
            ms_string += "\n.XE\n";
        }else if line.starts_with("### "){
            ms_string += ".NH 3\n";
            ms_string += &line[4..];
            ms_string += "\n.XS\n";
            ms_string += &line[4..];
            ms_string += "\n.XE\n";
        }else if line.starts_with("#### "){
            ms_string += ".NH 4\n";
            ms_string += &line[5..];
            ms_string += "\n.XS\n";
            ms_string += &line[5..];
            ms_string += "\n.XE\n";
        }else if line.starts_with("##### "){
            ms_string += ".NH 5\n";
            ms_string += &line[6..];
            ms_string += "\n.XS\n";
            ms_string += &line[6..];
            ms_string += "\n.XE\n";
        }else if line.starts_with("###### "){
            ms_string += ".NH 6\n";
            ms_string += &line[7..];
            ms_string += "\n.XS\n";
            ms_string += &line[7..];
            ms_string += "\n.XE\n";
        }else if line.starts_with("#Quote"){
            in_paragraph = true;
            in_quote = true;
            ms_string += ".B1\n.QP\n";
        }else if line.trim().is_empty(){
            in_paragraph = false;
            if in_quote {
                ms_string += "\n.B2\n";
                in_quote = false;
            }else{
                ms_string += "\n";
            }
        }

        else{
            if !in_paragraph {
                in_paragraph = true;
                ms_string += ".PP\n";
            }

            if line.contains("**") || line.contains("++"){
                let sub_strings = line.split("**");

                let mut is_bold = false;
                let mut is_italic = false;

                for s in sub_strings{
                    if is_bold {
                        ms_string += "\n.B ";
                    }

                    if s.contains("++"){
                        let sub_italics = s.split("++");

                        for i in sub_italics{
                            if is_italic {
                                ms_string += "\n.I ";
                            }

                            ms_string += i.trim_start();
                            if is_italic {
                                ms_string += "\n";
                            }

                            is_italic = !is_italic;
                        }
                    }else{
                        ms_string += s.trim_start();
                    }

                    if is_bold {
                        ms_string += "\n";
                    }

                    is_bold = !is_bold;
                }
            }else{
                ms_string += line;
            }

            if line.ends_with("  "){
                ms_string += "\n\n";
            }
        }
    }
    ms_string
}