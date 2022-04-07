
_DELTA = 0.01;

ONE_HP = 0.2*25.4;
ONE_U  = 39.65;
// approximately equal to a standard PCB thickness
THICKNESS = 1.6;
MODULE_WIDTH = 23*ONE_HP;

// the main facade body
//   Process: plot User Drawings layer from KiCAD (contains outline
//     and holes for jacks, encoders, etc). Open that in Inkscape,
//     process circles using "Object->Object to Path", then select path,
//     process using "Segments to lines", then save as R14 DXF to import here.
module main() {
    linear_extrude(height=THICKNESS)
        import(file="drawing.dxf");
}

module mounting_hole(x_offset=0, y_offset=0) {
    $fn = 36;
    MOUNTING_HOLE_DIAMETER = 3.2;
    translate([x_offset, y_offset, THICKNESS/2])
        hull() {
            translate([-1, 0, 0])
                cylinder(THICKNESS+_DELTA, MOUNTING_HOLE_DIAMETER/2, MOUNTING_HOLE_DIAMETER/2, center=true);
            translate([1, 0, 0])
                cylinder(THICKNESS+_DELTA, MOUNTING_HOLE_DIAMETER/2, MOUNTING_HOLE_DIAMETER/2, center=true);
        }
}

module three() {
    
}

difference() {  
    translate([1*ONE_HP, 0, 0])
        main();
    mounting_hole(7.5, 3);
    mounting_hole(7.5, ONE_U-3);
    mounting_hole(MODULE_WIDTH-7.5, 3);
    mounting_hole(MODULE_WIDTH-7.5, ONE_U-3);
}

// extra space to add support
cube([ONE_HP+10*_DELTA, ONE_U-0.2, THICKNESS]);
translate([2.5, 0, 0]) {
    // support
    support_height = 12;
    translate([ONE_HP/4, ONE_U/2, -support_height/2])
        cube([ONE_HP/2, ONE_U/2, support_height], center=true);
    translate([ONE_HP/2, ONE_U/2, -support_height-1])
        cube([ONE_HP, ONE_U/2, 2], center=true);
}