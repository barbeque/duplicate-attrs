extern crate xml;

use std::env;
use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

use xml::attribute::OwnedAttribute;
use xml::name::OwnedName;

fn print_usage() {
    println!("Usage: duplicate-attrs <xmlfile> [xmlfile2] ...")
}

struct DiscoveredDuplicate {
    pub element : String,
    pub attribute_name : String
}

fn detect_duplicates(element_name : &OwnedName, attrs : &Vec<OwnedAttribute>) -> Vec<DiscoveredDuplicate> {
    let mut result = Vec::<DiscoveredDuplicate>::new();

    for attr in attrs.iter() {
        let attrs_with_my_name = attrs.iter().filter(|&a| a.name == attr.name);
        if attrs_with_my_name.count() > 1  {
            // TODO: slightly inefficient since we'll hit this path at least twice if there's duplicates...
            if !result.iter().any(|x| x.attribute_name == attr.name.local_name) {
                result.push(
                    DiscoveredDuplicate { element: element_name.local_name.clone(), attribute_name: attr.name.local_name.clone() }
                )
            }
        }
    }

    result
}

fn check_buffer<R : std::io::Read>(buffer : R) {
    let parser = EventReader::new(buffer);

    // TODO: Maybe record the path we are working on instead of the element name.
    
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                // Check attributes for duplicates
                let dupes = detect_duplicates(&name, &attributes);
                for dupe in dupes {
                    println!("Found duplicate attr on element {}: '{}'", dupe.element, dupe.attribute_name);                    
                }
            }
            Err(e) => {
                println!("Error interpreting xml: {}", e);
                break;
            }
            _ => {}
        }
    }
}

fn check_file(file_path : &String) {
    let fp = File::open(file_path).unwrap();
    let fp = BufReader::new(fp);

    check_buffer(fp);
}

fn main() {
    // get the xml file we're pointing to
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            print_usage();
        },
        _ => {
            // load all the xml files
            for file in args.iter().skip(1) {
                println!("Reading '{}'", file);
                check_file(file);
            }
        }
    }
}
