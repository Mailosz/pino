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

    //log(format!("Orientations {}, {}, {}, {}", o1, o2, o3, o4).as_str());

    return o1 != o2 && o3 != o4
}

fn is_visible(a : &Pos, b : &Pos, c : &Pos, polygon_orientation : &Orientation, addresses: &Vec<usize>, polygon_points : &Vec<P>, vertices_left : usize) -> bool {

    //log(format!("Compute is visible for {}, {}, {}", a.index, b.index, c.index).as_str());

    if (!is_convex(&a.point, &b.point, &c.point, polygon_orientation)) {
        //log("111 Not convex");
        return false
    } else {
        let test = Orientation::Clockwise == Orientation::Clockwise;
        //log("BEGIN");
        // look for other line segemnts

        let mut index = addresses[c.index];
        let mut from = polygon_points[index];
        //log(format!("From {}", index).as_str());
        let mut o1 = get_orientation(&a.point, &b.point, &from);

        // check if new line is not within the angle (b, c, a)
        if is_convex(&b.point, &c.point, &from, polygon_orientation)
            && is_convex(&from, &c.point, &a.point, polygon_orientation)  {
            //log("Not visible - not convex at the end");
            return false
        }

        let mut index = addresses[index];
        //log(format!("To   {}", index).as_str());
        let mut to = polygon_points[index];

        // test whether any line intersects the new one
        if (vertices_left > 5) {
            loop {
                // check whether the line from, to intersects with the proposed line a,c
                let o2 = get_orientation(&a.point, &b.point, &to);
                
                if (o1 != o2)  {
                    let o3 = get_orientation(&from, &to, &a.point);
                    let o4 = get_orientation(&from, &to, &b.point);

                    if (o3 != o4) {
                        //disregard lines that only touch but not inersect
                        if (o1 != Orientation::Colinear && o2 != Orientation::Colinear && o3 != Orientation::Colinear && o4 != Orientation::Colinear) 
                        {
                            return false;
                        }
                    }
                }
                o1 = o2;

                // if (do_intersect(&a.point, &c.point, &from, &to)) {
                //     //log("Not visible - przecinają się");
                //     return false;
                // }
                index = addresses[index];
                //log(format!("Ind  {}", index).as_str());
                if (index == a.index) {
                    //log("koniec");
                    break;
                }
                
                from = to;
                to = polygon_points[index];
            }
        } else {
            //log("Less than 5");
        }
        //TODO: this is computed twice - now, and later in inner loop
        // check if new line is not within the angle (b, c, a)
        if is_convex(&b.point, &a.point, &to, polygon_orientation)
            && is_convex(&to, &a.point, &c.point, polygon_orientation)  {
            // //log(format!("Not visible - not convex at the end ({})", index).as_str());
            return false
        }
        
        return true
    }
}

fn is_in(a : P, b : P, c : P, d : P) -> bool{
    return ((c.x > a.x && c.x < b.x) || (c.x > b.x && c.x < a.x)) && ((c.y > a.y && c.y < b.y) || (c.y > b.y && c.y < a.y)) 
    || ((d.x > a.x && d.x < b.x) || (d.x > b.x && d.x < a.x)) && ((d.y > a.y && d.y < b.y) || (d.y > b.y && d.y < a.y)) 
}


fn get_line_intersection_point(a : P, b : P, c : P, d : P) -> Option<P> {

        let l1_w = b.x - a.x;     
        let l1_h = b.y - a.y;
        let l2_w = d.x - c.x;     
        let l2_h = d.y - c.y;
        let d_w = a.x - c.x;
        let d_h = a.y - c.y;
        
        let s = (-l1_h * (d_w) + l1_w * (d_h)) / (-l2_w * l1_h + l1_w * l2_h);
        let t = ( l2_w * (d_h) - l2_h * (d_w)) / (-l2_w * l1_h + l1_w * l2_h);
    
        if (s > 0.0 && s < 1.0 && t > 0.0 && t < 1.0)
        {
            return Some(P::new(a.x + (t * l1_w), a.y + (t * l1_h)));
        }
        return Option::None; // No collision
}


/**
 * Tesselates polygon using ear clipping method
 */
pub fn tesselate_polygon(polygon : &Polygon) -> Vec<Triangles> {


    let mut vertices_left = polygon.points.len();
    let mut index = Index{index: 0};
    let mut addresses : Vec<usize> = Vec::with_capacity(polygon.points.len());
    let mut strips : Vec<Triangles> = Vec::new();
    let mut current : Pos;

    for i in 1..polygon.points.len() {addresses.push(i)};
    addresses.push(0);


    let mut a = get_next(&polygon, &mut addresses, &mut index);
    let mut b = get_next(&polygon, &mut addresses, &mut index);
    let mut c = get_next(&polygon, &mut addresses, &mut index);


    let mut count = 0;
    'outer : loop {
        let visible = is_visible(&a, &b, &c, &polygon.orientation, &addresses, &polygon.points, vertices_left);
        
        if visible { // found possible triangle
            //TODO: check przecięcie z innymi liniami
            //log(format!("Visible {}, {}, {}", a.index, b.index, c.index).as_str());
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
                    //log("Finished with FAN");
                    break 'outer;
                }


                if !is_visible(&a, &b, &c, &polygon.orientation, &addresses, &polygon.points, vertices_left) {break};
                //log(format!("Visible {}, {}, {}", a.index, b.index, c.index).as_str());
            }
            //log(format!("FAN from {} to {}", a.index, b.index).as_str());
            strips.push(Triangles{vertices : strip, mode : TrianglesMode::Fan});
            addresses[a.index] = b.index;
            count = 0;
        } else {
            count += 1;
            if (count > vertices_left) {
                log("EMERGENCY TESSELEATION BREAK!!!");
                break;
            }
            //log(format!("Not visible {}, {}, {}", a.index, b.index, c.index).as_str());
        }
        a = b;
        b = c;
        c = get_next(&polygon, &mut addresses, &mut index);
    }

    strips

}



pub fn normalize_polygon(polygon : &mut Polygon) {



    let mut start = 1;

    loop {
        let line_start_point = polygon.points[start - 1];
        let mut index = start + 1;
        let mut second_line_start = polygon.points[index];
        loop {
            index += 1;
            if (index == polygon.points.len()) {
                //log(format!("Break in {}", index).as_str());

                let intersection_point = get_line_intersection_point(line_start_point, polygon.points[start], second_line_start, polygon.points[0]);

                match intersection_point {
                    Some(p) => intersection_found(start, index, p, polygon),
                    None => ()// //log(format!("Not {}, {}", start, index).as_str())
                }
                break;
            }
            let second_line_end = polygon.points[index];
            let intersection_point = get_line_intersection_point(line_start_point, polygon.points[start], second_line_start, second_line_end);

            match intersection_point {
                Some(p) => intersection_found(start, index, p, polygon),
                None => ()//alog(format!("Not {}, {}", start, index).as_str())
            }

            second_line_start = second_line_end;
        }

        start += 1;
        if (start + 1 == polygon.points.len()) {
            // //log(format!("Break out {}", start).as_str());
            break;
        }
        // //log("continue");
    }
}

fn intersection_found(first: usize, second: usize, point : P, polygon: &mut Polygon) {
    
    //log(format!("Intersection found between segments {} and {} at point {}, {}", first, second, point.x, point.y).as_str());
    let mut a = first;
    let mut b = second - 1;
    loop {
        let var = polygon.points[b];
        polygon.points[b] = polygon.points[a];
        polygon.points[a] = var;

        a += 1;
        b -= 1;

        if (a >= b) {
            break;
        }
        
    }

    polygon.points.insert(second, point);
    polygon.points.insert(first, point);

}