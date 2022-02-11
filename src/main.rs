use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_args(&args);
    let contents: String = read_contents(&filename);

    let out_filename  =  &filename[..(filename.len()-2)];
    let out_pdf = String::from(out_filename) + "pdf";
    let out_filename = String::from(out_filename) + "ms";

    let mut out_file = std::fs::File::create(&out_filename).expect("Can't create the ms file!");
    out_file.write_all(".R1\naccumulate\ndatabase bib.ref\nmove-punctuation\n.R2\n\n".as_bytes())
        .expect("Writing to output file failed!");

    let mut in_paragraph: bool = false;
    for line in contents.lines(){
        if line.starts_with("#T "){
            out_file.write_all(".TL\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[3..].as_bytes()).expect("Can't write output!");
        }else if line.starts_with("#A "){
            out_file.write_all("\n.AU\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[3..].as_bytes()).expect("Can't write output!");
        }else if line.starts_with("#I "){
            out_file.write_all("\n.AI\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[3..].as_bytes()).expect("Can't write output!");
        }else if line.starts_with("#Date"){
            out_file.write_all(".DA\n".as_bytes()).expect("Can't write output!");
        }else if line.starts_with("#Break"){
            out_file.write_all(".bp\n".as_bytes()).expect("Can't write output!");
        }else if line.starts_with("#AbstractBegin"){
            out_file.write_all(".AB\n".as_bytes()).expect("Can't write output!");
        }else if line.starts_with("#AbstractEnd"){
            out_file.write_all("\n.AE\n".as_bytes()).expect("Can't write output!");
        }else if line.starts_with("# "){
            out_file.write_all(".NH 1\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[2..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XS\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[2..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XE\n".as_bytes()).expect("Can't write output!");
        }else if line.starts_with("## "){
            out_file.write_all(".NH 2\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[3..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XS\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[3..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XE\n".as_bytes()).expect("Can't write output!");
        }else if line.starts_with("### "){
            out_file.write_all(".NH 3\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[4..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XS\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[4..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XE\n".as_bytes()).expect("Can't write output!");
        }else if line.starts_with("#### "){
            out_file.write_all(".NH 4\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[5..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XS\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[5..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XE\n".as_bytes()).expect("Can't write output!");
        }else if line.starts_with("##### "){
            out_file.write_all(".NH 5\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[6..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XS\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[6..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XE\n".as_bytes()).expect("Can't write output!");
        }else if line.starts_with("###### "){
            out_file.write_all(".NH 6\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[7..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XS\n".as_bytes()).expect("Can't write output!");
            out_file.write_all(line[7..].as_bytes()).expect("Can't write output!");
            out_file.write_all("\n.XE\n".as_bytes()).expect("Can't write output!");
        }else if line.starts_with("#Quote"){
            in_paragraph = true;
            out_file.write_all(".QP\n".as_bytes()).expect("Can't write output!");
        }else if line.trim().is_empty(){
            in_paragraph = false;
            out_file.write_all("\n".as_bytes()).expect("Can't write output!");
        }

        else{
            if !in_paragraph {
                in_paragraph = true;
                out_file.write_all(".PP\n".as_bytes()).expect("Can't write output!");
            }

            out_file.write_all(line.as_bytes()).expect("Can't write output!");
            if line.ends_with("  "){
                out_file.write_all("\n\n".as_bytes()).expect("Can't write output!");
            }
        }
    }

    create_pdf(&out_filename, &out_pdf);
}

fn create_pdf(ms_file: &String, pdf_file: &String){
    let output = Command::new("groff")
        .arg("-mspdf")
        .arg("-Tpdf")
        .arg(ms_file)
        .output()
        .expect("Failed to call groff. Make sure groff is installed!");

        let mut out_pdf = std::fs::File::create(&pdf_file).expect("Can't create the pdf file!");
        out_pdf.write_all(output.stdout.as_slice()).expect("Can't write to pdf file!");
}

fn parse_args(args: &[String]) -> &String {
    &args[1]
}

fn read_contents(filename: &String) -> String {
    fs::read_to_string(filename)
        .expect("Can't open the file!")
}