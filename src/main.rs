use std::env;
use std::fs;
use std::io::Write;
use std::process::Command;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_args(&args);

    if filename == "init" {
        initialize();
        return;
    }
    
    let out_filename  =  &filename[..(filename.len()-2)];
    let out_pdf = String::from(out_filename) + "pdf";
    let out_ps = String::from(out_filename) + "ps";
    //let out_html = String::from(out_filename) + "html";
    let out_filename = String::from(out_filename) + "ms";

    let ms_string = create_ms(&filename);
    write_ms(&ms_string, &out_filename);

    convert_pictures(&out_filename);
    create_ps(&out_filename, &out_ps);
    //create_html(&out_filename, &out_html);
    create_pdf(&out_ps, &out_pdf);
}

fn initialize(){
    fs::create_dir("pic").expect("Can't create picture Directory \"pic\" consider creating it manually!");
    fs::create_dir("code").expect("Can't create the \"code\" directory! Create it manually!");

    let mut out_string = String::from("#T A dropMarkdown file!\n");
    out_string += "#A Droplet is the author\n";
    out_string += "#I dropsoft.org is the institution\n";
    out_string += "\n# This is a first heading!\n";
    out_string += "This is a first paragraph\n";
    out_string += "~ref-id\n";

    let mut out_file = std::fs::File::create("text.dm").expect("Can't create the dm file! ;(");
    out_file.write_all(out_string.as_bytes()).expect("Can't write to dm file :/");

    let mut out_string = String::from("%K ref-id\n");
    out_string += "%D 2022\n";
    out_string += "%T The Complete Title\n";
    out_string += "%A Droplet\n";
    out_string += "%O https://dropsoft.org\n";
    out_string += "%I DropSoft\n";
    out_string += "%G ISBN 666666666";


    let mut out_file = std::fs::File::create("bib.ref").expect("Can't create bib.ref! ;(");
    out_file.write_all(out_string.as_bytes()).expect("Can't write to ref file :/");
}

fn convert_pictures(out_filename: &String) {
    let file_path = Path::new(out_filename).parent().unwrap();
    let subdir = "pic";
    let file_path = file_path.join(subdir);
    
    if !file_path.exists() {
        print!("Picture path was not found!\n");
        return;
    }

    for entry in file_path.read_dir().expect("can't read pictures!") {
        if let Ok(entry) = entry {
            if entry.path().extension().unwrap() == "eps" {
                continue;
            }

            Command::new("convert")
                .arg(entry.path())
                .arg(entry.path().with_extension("eps"))
                .output()
                .expect("Failed to convert image!");
        }
    }
}

fn create_ps(ms_file: &String, ps_file: &String){
    let output = Command::new("groff")
        .arg("-ms")
        .arg("-Tps")
        .arg("-s")
        .arg("-e")
        .arg("-p")
        .arg("-t")
        .arg("-R")
        .arg("-K")
        .arg("utf-8")
        .arg("-k")
        .arg(ms_file)
        .output()
        .expect("Failed to call groff. Make sure groff is installed!");

        println!("{}", String::from_utf8(output.stderr).unwrap());
        let mut out_ps = std::fs::File::create(&ps_file).expect("Can't create the ps file!");
        out_ps.write_all(output.stdout.as_slice()).expect("Can't write to ps file!");
}

/*
fn create_html(ms_file: &String, html_file: &String){
    let output = Command::new("groff")
        .arg("-ms")
        .arg("-Thtml")
        .arg("-s")
        .arg("-e")
        .arg("-p")
        .arg("-t")
        .arg("-R")
        .arg(ms_file)
        .output()
        .expect("Failed to call groff. Make sure groff is installed!");

        let mut out_ps = std::fs::File::create(&html_file).expect("Can't create the html file!");
        out_ps.write_all(output.stdout.as_slice()).expect("Can't write to html file!");
}*/

fn create_pdf(ps_file: &String, pdf_file: &String){
    Command::new("ps2pdf")
        .arg(ps_file)
        .arg(pdf_file)
        .output()
        .expect("Failed to call groff. Make sure groff is installed!");
}

fn write_ms(ms_string : &String, ms_file : &String){
    let mut out_file = std::fs::File::create(&ms_file).expect("Can't create the ms file! ;(");
    out_file.write_all(ms_string.as_bytes()).expect("Can't write to ms file :/");
}

fn parse_args(args: &[String]) -> &String {
    if args.len() != 2 {
        println!("Usage: dropMarkdown [ init | <file.dm> ]");
        exit(0);
    }
    &args[1]
}

fn read_contents(filename: &String) -> String {
    fs::read_to_string(filename)
        .expect("Can't open the file!")
}

fn color_code(code_file: &String) -> String {
    let lang = code_file.split_terminator('.').last().unwrap();

    let output = Command::new("source-highlight")
        .arg("-s")
        .arg(lang)
        .arg("-i")
        .arg(code_file)
        .arg("-o")
        .arg("/dev/stdout")
        .arg("--out-format")
        .arg("groff_mm_color")
        .output()
        .expect("Failed to call groff. Make sure groff is installed!");

        println!("{}", String::from_utf8(output.stderr).unwrap());
        String::from_utf8(output.stdout).unwrap()
}

fn create_ms(drop_file: &String) -> String {
    let contents: String = read_contents(&drop_file);
    let mut ms_string = String::new();

    ms_string += ".R1\naccumulate\n\ndatabase bib.ref\n\nmove-punctuation\n\n.R2\n\n";
    //ms_string += ".ds N \\\\fB\\\\n+n.\\\\fR\n";
    //ms_string += ".ps 20\n.vs 24\n.fam HN\n\n";
    //ms_string += ".bp";

    //cover page
    //ms_string += ".PSPIC -C \"./design/cover.eps\n";

    //Add logo:
    ms_string += ".PSPIC -C \"./design/logo.eps\n\n";

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
        }else if line.starts_with("#MathBegin"){
            ms_string += "\n.EQ\n";
        }else if line.starts_with("#MathEnd"){
            ms_string += "\n.EN\n";
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
        }else if line.starts_with("#Picture"){
            let mut pic_subs = line.split_ascii_whitespace();
            pic_subs.next(); //#Picture
            let pic_path = pic_subs.next();
            
            ms_string += ".PSPIC -C \"./pic/";
            ms_string += pic_path.unwrap();
            ms_string += ".eps\" 5i 5i \n";
            ms_string += ".ce\n";

            for word in pic_subs {
                ms_string += word;
                ms_string += " "
            }
            ms_string += "\n\n";

        }else if line.starts_with("#Quote"){
            in_paragraph = true;
            in_quote = true;
            ms_string += ".B1\n.QP\n";
        }else if line.starts_with("~"){
            let reference = line.strip_prefix("~").unwrap();
            ms_string += "\n.[\n";
            ms_string += reference;
            ms_string += "\n.]\n";
        }else if line.starts_with("#List"){
            ms_string += "\n.[\n";
            ms_string += "$LIST$";
            ms_string += "\n.]\n";
        }else if line.starts_with("#Code"){
            let mut code_subs = line.split_ascii_whitespace();
            code_subs.next(); //#Picture
            let code_path = String::from(code_subs.next().unwrap());
            let colored_code = color_code(&code_path);

            ms_string += ".B1\n";
            ms_string += &colored_code;
            ms_string += "\n\\m[]\n.B2\\m[]\n";

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