use std::f64::consts::PI;

/// Convert a series of (x, y) pixel coordinates to an SVG path `d` attribute string.
pub fn points_to_path_d(points: &[(f64, f64)]) -> String {
    if points.is_empty() {
        return String::new();
    }
    let mut d = format!("M {:.2} {:.2}", points[0].0, points[0].1);
    for &(x, y) in &points[1..] {
        d.push_str(&format!(" L {:.2} {:.2}", x, y));
    }
    d
}

/// Convert points to a smooth SVG path using monotone cubic interpolation.
pub fn points_to_smooth_path_d(points: &[(f64, f64)]) -> String {
    if points.len() < 2 {
        return points_to_path_d(points);
    }
    if points.len() == 2 {
        return points_to_path_d(points);
    }

    let n = points.len();
    let mut d = format!("M {:.2} {:.2}", points[0].0, points[0].1);

    for i in 1..n {
        let p0 = if i > 1 { points[i - 2] } else { points[i - 1] };
        let p1 = points[i - 1];
        let p2 = points[i];
        let p3 = if i + 1 < n { points[i + 1] } else { points[i] };

        // Catmull-Rom to cubic bezier control points
        let tension = 6.0;
        let cp1x = p1.0 + (p2.0 - p0.0) / tension;
        let cp1y = p1.1 + (p2.1 - p0.1) / tension;
        let cp2x = p2.0 - (p3.0 - p1.0) / tension;
        let cp2y = p2.1 - (p3.1 - p1.1) / tension;

        d.push_str(&format!(
            " C {:.2} {:.2}, {:.2} {:.2}, {:.2} {:.2}",
            cp1x, cp1y, cp2x, cp2y, p2.0, p2.1
        ));
    }
    d
}

/// Create a closed area path: line path + vertical drop to baseline + close.
pub fn area_path_d(points: &[(f64, f64)], baseline_y: f64, smooth: bool) -> String {
    if points.is_empty() {
        return String::new();
    }

    let line = if smooth {
        points_to_smooth_path_d(points)
    } else {
        points_to_path_d(points)
    };

    let last = points.last().unwrap();
    let first = points.first().unwrap();

    format!(
        "{} L {:.2} {:.2} L {:.2} {:.2} Z",
        line, last.0, baseline_y, first.0, baseline_y
    )
}

/// Generate an SVG arc path segment for a pie/donut slice.
///
/// Returns a path `d` string for the slice from `start_angle` to `end_angle`
/// (in radians, 0 = top, clockwise).
pub fn arc_path(
    cx: f64,
    cy: f64,
    outer_radius: f64,
    inner_radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> String {
    let angle_offset = -PI / 2.0; // Start from top

    let sa = start_angle + angle_offset;
    let ea = end_angle + angle_offset;

    let outer_start_x = cx + outer_radius * sa.cos();
    let outer_start_y = cy + outer_radius * sa.sin();
    let outer_end_x = cx + outer_radius * ea.cos();
    let outer_end_y = cy + outer_radius * ea.sin();

    let large_arc = if (end_angle - start_angle) > PI {
        1
    } else {
        0
    };

    if inner_radius <= 0.0 {
        // Pie slice (no hole)
        format!(
            "M {:.2} {:.2} L {:.2} {:.2} A {:.2} {:.2} 0 {} 1 {:.2} {:.2} Z",
            cx, cy, outer_start_x, outer_start_y, outer_radius, outer_radius, large_arc,
            outer_end_x, outer_end_y
        )
    } else {
        // Donut slice
        let inner_start_x = cx + inner_radius * sa.cos();
        let inner_start_y = cy + inner_radius * sa.sin();
        let inner_end_x = cx + inner_radius * ea.cos();
        let inner_end_y = cy + inner_radius * ea.sin();

        format!(
            "M {:.2} {:.2} A {:.2} {:.2} 0 {} 1 {:.2} {:.2} L {:.2} {:.2} A {:.2} {:.2} 0 {} 0 {:.2} {:.2} Z",
            outer_start_x, outer_start_y,
            outer_radius, outer_radius, large_arc,
            outer_end_x, outer_end_y,
            inner_end_x, inner_end_y,
            inner_radius, inner_radius, large_arc,
            inner_start_x, inner_start_y
        )
    }
}

/// Generate an SVG arc path for a pie/donut slice, always split into two
/// sub-arcs.  This ensures the path command structure (number and type of
/// commands) is identical for every slice regardless of sweep angle, which
/// is critical for smooth CSS `d` property transitions — the browser can
/// interpolate matching commands component-by-component without the
/// large-arc-flag flipping mid-transition.
///
/// Pie:   M, L, A, A, Z   (5 commands, large_arc always 0)
/// Donut: M, A, A, L, A, A, Z  (7 commands, large_arc always 0)
pub fn arc_path_split(
    cx: f64,
    cy: f64,
    outer_radius: f64,
    inner_radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> String {
    let offset = -PI / 2.0;
    let sa = start_angle + offset;
    let ma = (start_angle + end_angle) / 2.0 + offset; // midpoint
    let ea = end_angle + offset;

    let ox = |a: f64| cx + outer_radius * a.cos();
    let oy = |a: f64| cy + outer_radius * a.sin();

    if inner_radius <= 0.0 {
        // Pie: M center → L outer_start → A outer_mid → A outer_end → Z
        format!(
            "M {:.2} {:.2} L {:.2} {:.2} A {:.2} {:.2} 0 0 1 {:.2} {:.2} A {:.2} {:.2} 0 0 1 {:.2} {:.2} Z",
            cx, cy,
            ox(sa), oy(sa),
            outer_radius, outer_radius, ox(ma), oy(ma),
            outer_radius, outer_radius, ox(ea), oy(ea),
        )
    } else {
        // Donut: outer start → A outer mid → A outer end → L inner end → A inner mid → A inner start → Z
        let ix = |a: f64| cx + inner_radius * a.cos();
        let iy = |a: f64| cy + inner_radius * a.sin();

        format!(
            "M {:.2} {:.2} A {:.2} {:.2} 0 0 1 {:.2} {:.2} A {:.2} {:.2} 0 0 1 {:.2} {:.2} L {:.2} {:.2} A {:.2} {:.2} 0 0 0 {:.2} {:.2} A {:.2} {:.2} 0 0 0 {:.2} {:.2} Z",
            ox(sa), oy(sa),
            outer_radius, outer_radius, ox(ma), oy(ma),
            outer_radius, outer_radius, ox(ea), oy(ea),
            ix(ea), iy(ea),
            inner_radius, inner_radius, ix(ma), iy(ma),
            inner_radius, inner_radius, ix(sa), iy(sa),
        )
    }
}

/// Convert client mouse coordinates to SVG viewBox coordinates.
///
/// Uses the SVG element's CTM (current transformation matrix) for robust
/// conversion regardless of CSS transforms, scaling, or viewport changes.
pub fn client_to_svg(
    svg_el: &web_sys::SvgsvgElement,
    client_x: f64,
    client_y: f64,
) -> Option<(f64, f64)> {
    let pt = svg_el.create_svg_point();
    pt.set_x(client_x as f32);
    pt.set_y(client_y as f32);

    let ctm = svg_el.get_screen_ctm()?;
    let inv = ctm.inverse().ok()?;
    let svg_pt = pt.matrix_transform(&inv);

    Some((svg_pt.x() as f64, svg_pt.y() as f64))
}
