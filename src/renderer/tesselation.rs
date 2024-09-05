use crate::{base::log, renderer::{TrianglesMode, P}, Orientation};

use super::{Polygon, Triangles};

struct Pos {
    index : usize,
    point : P,
}

struct Index {
    pub index : usize
}

fn get_next(polygon : &Polygon, addresses: &mut Vec<usize>, index : &mut Index) -> Pos {
    index.index = addresses[index.index];
    Pos{index : index.index, point : polygon.points[index.index]}
}


/**
 * Checks whether angle abc is convex, and if the point c is visible from point a 
 */
fn is_convex(a : &P, b : &P, c : &P, polygon_orientation : &Orientation) -> bool {
    let orientation = get_orientation(&a, &b, &c);

    match polygon_orientation {
        Orientation::Clockwise => orientation == Orientation::Clockwise,
        Orientation::CounterClockwise => orientation == Orientation::CounterClockwise,
        Orientation::Colinear => panic!("Polygon cannot be collinear!")
    }
}


fn get_orientation(a : &P, b : &P, c : &P) -> Orientation
{ 
    let orientation = (b.y - a.y) * (c.x - b.x) -  (b.x - a.x) * (c.y - b.y); 
    if (orientation < 0.0) {
        Orientation::Clockwise
    } else if (orientation > 0.0) {
        Orientation::CounterClockwise
    } else {
        Orientation::Colinear
    }
} 

fn do_intersect(a : &P, b : &P, c : &P, d : &P) -> bool {
    let o1 = get_orientation(a, b, c);
    let o2 = get_orientation(a, b, d);
    let o3 = get_orientation(c, d, a);
    let o4 = get_orientation(c, d, b);

    log(format!("Orientations {}, {}, {}, {}", o1, o2, o3, o4).as_str());

    return o1 != o2 && o3 != o4
}

fn is_visible(start : usize, a : &Pos, b : &Pos, c : &Pos, polygon_orientation : &Orientation, addresses: &Vec<usize>, polygon_points : &Vec<P>, vertices_left : usize) -> bool {

    log(format!("Compute is visible for {}, {}, {}, {}", start, a.index, b.index, c.index).as_str());

    if (!is_convex(&a.point, &b.point, &c.point, polygon_orientation)) {
        return false
    } else {
        let test = Orientation::Clockwise == Orientation::Clockwise;
        log("BEGIN");
        log(format!("Start: {}", start).as_str());
        // look for other line segemnts

        let mut index = addresses[c.index];
        let mut from = polygon_points[index];
        log(format!("From {}", index).as_str());

        // check if new line is not within the angle (b, c, a)
        if is_convex(&b.point, &c.point, &from, polygon_orientation)
            && is_convex(&from, &c.point, &a.point, polygon_orientation)  {
            log("Not visible - not convex at the start");
            return false
        }

        let mut index = addresses[index];
        log(format!("To   {}", index).as_str());
        let mut to = polygon_points[index];

        // test whether any line intersects the new one
        if (vertices_left > 5) {
            loop {
                if (do_intersect(&a.point, &c.point, &from, &to)) {
                    log("Not visible - przecinają się");
                    return false;
                }
                index = addresses[index];
                log(format!("Ind  {}", index).as_str());
                if (index == a.index) {
                    log("koniec");
                    break;
                }
                
                from = to;
                to = polygon_points[index];
            }
        } else {
            log("Less than 5");
        }
        //TODO: this is computed twice - now, and later in inner loop
        // check if new line is not within the angle (b, c, a)
        if is_convex(&b.point, &c.point, &to, polygon_orientation)
            && is_convex(&to, &c.point, &a.point, polygon_orientation)  {
            log("Not visible - not convex at the start");
            return false
        }
        
        return true
    }
}


/**
 * Tesselates polygon using ear clipping method
 */
pub fn tesselate_polygon(polygon : &Polygon) -> Vec<Triangles> {


    let mut vertices_left = polygon.points.len();
    let mut start = 0;
    let mut index = Index{index: start};
    let mut addresses : Vec<usize> = Vec::with_capacity(polygon.points.len());
    let mut strips : Vec<Triangles> = Vec::new();
    let mut current : Pos;

    for i in 1..polygon.points.len() {addresses.push(i)};
    addresses.push(0);


    let mut a = get_next(&polygon, &mut addresses, &mut index);
    let mut b = get_next(&polygon, &mut addresses, &mut index);
    let mut c = get_next(&polygon, &mut addresses, &mut index);


    
    'outer : loop {
        let visible = is_visible(start, &a, &b, &c, &polygon.orientation, &addresses, &polygon.points, vertices_left);
        
        if visible { // found possible triangle
            //TODO: check przecięcie z innymi liniami
            log(format!("Visible {}, {}, {}", a.index, b.index, c.index).as_str());
            let mut strip : Vec<f32> = Vec::new();
            strip.push(a.point.x());
            strip.push(a.point.y());
            strip.push(b.point.x());
            strip.push(b.point.y());
            

            loop {
                strip.push(c.point.x());
                strip.push(c.point.y());

                
                vertices_left -= 1;
                b = c;
                c = get_next(&polygon, &mut addresses, &mut index);

                if (vertices_left < 4) {

                    strip.push(c.point.x());
                    strip.push(c.point.y());
                    strips.push(Triangles{vertices : strip, mode : TrianglesMode::Fan});
                    log("Finished with FAN");
                    break 'outer;
                }


                if !is_visible(start, &a, &b, &c, &polygon.orientation, &addresses, &polygon.points, vertices_left) {break};
                log(format!("Visible {}, {}, {}", a.index, b.index, c.index).as_str());
            }
            log(format!("FAN from {} to {}", a.index, b.index).as_str());
            strips.push(Triangles{vertices : strip, mode : TrianglesMode::Fan});
            addresses[a.index] = b.index;
        } else {
            log(format!("Not visible {}, {}, {}", a.index, b.index, c.index).as_str());
        }
        start = a.index;
        a = b;
        b = c;
        c = get_next(&polygon, &mut addresses, &mut index);
    }

    strips

}



pub fn normalize_polygon(polygon : Polygon) {

}