extern crate rand;
use rand::{thread_rng, Rng};
extern crate clap;
use clap::Parser;
use std::fs::File;
use std::io::Write;

const MED_OFFSET: usize = 4;
const SMALL_OFFSET: usize = 6;

/// Generates square, randomized, 3-level, multi-scale truchet pattern svgs. Based on <https://christophercarlson.com/portfolio/multi-scale-truchet-patterns/>
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Width of the output svg in small tiles. Must be greater than 4.
    #[clap(short, long, default_value_t = 16)]
    width: usize,

    /// Output file name
    #[clap(short, long, default_value = "output.svg")]
    output: String,
}

fn main() {
    let args = Args::parse();
    generate_tiles(args);
}

fn generate_tiles(args: Args) {
    let width = args.width;
    if width < 4 {
        eprintln!("ERROR: \"width\" must be greater than 4.");
        std::process::exit(1);
    }
    let mut slots = vec![false; width * width]; //: [bool; width*width] = [false; width * width];
    let mut rng = thread_rng();
    let mut inner_svg = String::new();

    let num_large_across = width - 3;
    let mut x_large: usize = 0;
    'large: loop {
        if x_large >= (num_large_across * num_large_across) {
            break 'large;
        }
        //println!("{}", x_large);
        if can_place_large(x_large, &slots, width) {
            let should_add = rng.gen_bool(1.0 / 2.0);
            if should_add {
                let should_flip = rng.gen_bool(1.0 / 2.0);
                let column = x_large % num_large_across;
                let row = x_large / num_large_across;
                inner_svg.push_str(
                    get_tile_svg_string(Tile::Large, column * 6, row * 6, should_flip).as_str(),
                );
                let slots_to_take = get_slots_for_large(x_large, width);
                for slot in slots_to_take {
                    slots[slot] = true;
                }
            }
        }
        x_large = x_large + 1;
    }

    let num_med_across: usize = width - 1;
    let mut x_med: usize = 0;
    'med: loop {
        if x_med >= (num_med_across * num_med_across) {
            break 'med;
        }
        if can_place_med(x_med, &slots, width) {
            let should_add = rng.gen_bool(1.0 / 2.0);
            if should_add {
                let should_flip = rng.gen_bool(1.0 / 2.0);
                let column = x_med % num_med_across;
                let row = x_med / num_med_across;
                inner_svg.push_str(
                    get_tile_svg_string(
                        Tile::Medium,
                        (column * 6) + MED_OFFSET,
                        (row * 6) + MED_OFFSET,
                        should_flip,
                    )
                    .as_str(),
                );
                let slots_to_take = get_slots_for_med(x_med, width);
                for slot in slots_to_take {
                    slots[slot] = true;
                }
            }
        }
        x_med = x_med + 2;
    }

    let mut x_small: usize = 0;
    'small: loop {
        if x_small == width * width {
            break 'small;
        }

        if can_place_small(x_small, &slots) {
            let should_flip = rng.gen_bool(1.0 / 2.0);
            let row = x_small / width;
            let column = x_small % width;
            inner_svg.push_str(
                get_tile_svg_string(
                    Tile::Small,
                    (column * 6) + SMALL_OFFSET,
                    (row * 6) + SMALL_OFFSET,
                    should_flip,
                )
                .as_str(),
            );
        }

        x_small = x_small + 1;
    }
    let mut output_file = File::create(args.output).expect("");
    let result = write!(output_file, "<svg width=\"{0}\" height=\"{0}\" version=\"1.1\" xmlns=\"http://www.w3.org/2000/svg\">{1}</svg>", (width*6)+16, inner_svg);
    match result {
        Ok(()) => (),
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    };
}

fn can_place_large(large_slot: usize, slots: &[bool], width: usize) -> bool {
    let num_across = width - 3;
    let column = large_slot % num_across;
    let row = large_slot / num_across;
    if row % 4 != 0 || column % 4 != 0 {
        return false;
    }
    let requested = get_slots_for_large(large_slot, width);
    for r in requested {
        if slots[r] {
            return false;
        }
    }
    return true;
}

fn get_slots_for_large(large_slot: usize, width: usize) -> [usize; 16] {
    let num_across = width - 3;
    let column = large_slot % num_across;
    let row = large_slot / num_across;
    let base = (row * width) + column;
    let base_1 = base + width;
    let base_2 = base + (2 * width);
    let base_3 = base + (3 * width);
    [
        base,
        base + 1,
        base + 2,
        base + 3,
        base_1,
        base_1 + 1,
        base_1 + 2,
        base_1 + 3,
        base_2,
        base_2 + 1,
        base_2 + 2,
        base_2 + 3,
        base_3,
        base_3 + 1,
        base_3 + 2,
        base_3 + 3,
    ]
}

fn can_place_med(medium_slot: usize, slots: &[bool], width: usize) -> bool {
    let num_across = width - 1;
    let column = medium_slot % num_across;
    let row = medium_slot / num_across;
    if row % 2 != 0 || column % 2 != 0 {
        return false;
    }
    let requested = get_slots_for_med(medium_slot, width);
    for r in requested {
        if slots[r] {
            return false;
        }
    }
    return true;
}

fn get_slots_for_med(med_slot: usize, width: usize) -> [usize; 4] {
    let num_across = width - 1;
    let column = med_slot % num_across;
    let row = med_slot / num_across;
    let base = (row * width) + column;
    let base_1 = base + width;
    [base, base + 1, base_1, base_1 + 1]
}

fn can_place_small(small_slot: usize, slots: &[bool]) -> bool {
    !slots[small_slot]
}

enum Tile {
    Small,
    Medium,
    Large,
}

fn get_tile_svg_string(t: Tile, x_offset: usize, y_offset: usize, should_flip: bool) -> String {
    let scale: usize = match t {
        Tile::Small => 1,
        Tile::Medium => 2,
        Tile::Large => 4,
    };
    let color1 = match t {
        Tile::Small => "white",
        Tile::Medium => "black",
        Tile::Large => "white",
    };
    let color2 = match t {
        Tile::Small => "black",
        Tile::Medium => "white",
        Tile::Large => "black",
    };

    if should_flip {
        return format!(
            "\
        <rect width=\"{2}\" height=\"{2}\" x=\"{6}\" y=\"{11}\" fill=\"{0}\"/>\
        <path d=\"M {10} {11} L {10} {14} A {3} {3} 0 0 1 {7} {11}\" fill=\"{1}\"/>\
        <path d=\"M {6} {15} L {6} {12} A {3} {3} 0 0 1 {9} {15}\" fill=\"{1}\"/>\
        <circle cx=\"{6}\" cy=\"{13}\" r=\"{4}\" fill=\"{1}\"/>\
        <circle cx=\"{10}\" cy=\"{13}\" r=\"{4}\" fill=\"{1}\"/>\
        <circle cx=\"{8}\" cy=\"{11}\" r=\"{4}\" fill=\"{1}\"/>\
        <circle cx=\"{8}\" cy=\"{15}\" r=\"{4}\" fill=\"{1}\"/>\
        <circle cx=\"{6}\" cy=\"{11}\" r=\"{5}\" fill=\"{0}\"/>\
        <circle cx=\"{10}\" cy=\"{15}\" r=\"{5}\" fill=\"{0}\"/>\
        <circle cx=\"{6}\" cy=\"{15}\" r=\"{5}\" fill=\"{0}\"/>\
        <circle cx=\"{10}\" cy=\"{11}\" r=\"{5}\" fill=\"{0}\"/>\
        ",
            color1,
            color2,
            6 * scale,
            4 * scale,
            scale,
            2 * scale,
            (scale * 2) + x_offset,
            (scale * 4) + x_offset,
            (scale * 5) + x_offset,
            (scale * 6) + x_offset,
            (scale * 8) + x_offset,
            (scale * 2) + y_offset,
            (scale * 4) + y_offset,
            (scale * 5) + y_offset,
            (scale * 6) + y_offset,
            (scale * 8) + y_offset
        );
    }

    format!(
        "\
    <rect width=\"{2}\" height=\"{2}\" x=\"{6}\" y=\"{11}\" fill=\"{0}\"/>\
    <path d=\"M {6} {11} L {9} {11} A {3} {3} 0 0 1 {6} {14}\" fill=\"{1}\"/>\
    <path d=\"M {10} {15} L {7} {15} A {3} {3} 0 0 1 {10} {12}\" fill=\"{1}\"/>\
    <circle cx=\"{6}\" cy=\"{13}\" r=\"{4}\" fill=\"{1}\"/>\
    <circle cx=\"{10}\" cy=\"{13}\" r=\"{4}\" fill=\"{1}\"/>\
    <circle cx=\"{8}\" cy=\"{11}\" r=\"{4}\" fill=\"{1}\"/>\
    <circle cx=\"{8}\" cy=\"{15}\" r=\"{4}\" fill=\"{1}\"/>\
    <circle cx=\"{6}\" cy=\"{11}\" r=\"{5}\" fill=\"{0}\"/>\
    <circle cx=\"{10}\" cy=\"{15}\" r=\"{5}\" fill=\"{0}\"/>\
    <circle cx=\"{6}\" cy=\"{15}\" r=\"{5}\" fill=\"{0}\"/>\
    <circle cx=\"{10}\" cy=\"{11}\" r=\"{5}\" fill=\"{0}\"/>\
    ",
        color1,
        color2,
        6 * scale,
        4 * scale,
        scale,
        2 * scale,
        (scale * 2) + x_offset,
        (scale * 4) + x_offset,
        (scale * 5) + x_offset,
        (scale * 6) + x_offset,
        (scale * 8) + x_offset,
        (scale * 2) + y_offset,
        (scale * 4) + y_offset,
        (scale * 5) + y_offset,
        (scale * 6) + y_offset,
        (scale * 8) + y_offset
    )
}
