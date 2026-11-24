use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct LabelPosition {
    pub x: i32,
    pub y: i32,
    pub anchor: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct ViewBox {
    left: f64,
    top: f64,
    right: f64,
}

#[derive(Debug, Clone, Copy)]
struct Segment {
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    mid_x: f64,
    mid_y: f64,
}

impl Segment {
    fn straight(x1: f64, y1: f64, x2: f64, y2: f64) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            mid_x: (x1 + x2) / 2.0,
            mid_y: (y1 + y2) / 2.0,
        }
    }

    fn length_sq(&self) -> f64 {
        let dx = self.x2 - self.x1;
        let dy = self.y2 - self.y1;
        dx * dx + dy * dy
    }
}

/// Computes a readable SVG `<text>` position along an edge path `d`,
/// keeping the label inside `view_box` when bounds are present.
pub fn edge_label_position(d: &str, label: &str, view_box: &str) -> LabelPosition {
    let segments = parse_segments(d);
    let Some(best) = segments
        .into_iter()
        .max_by(|a, b| a.length_sq().total_cmp(&b.length_sq()))
    else {
        return LabelPosition {
            x: 0,
            y: 12,
            anchor: "start",
        };
    };

    place_on_segment(best, label, parse_view_box(view_box))
}

fn place_on_segment(seg: Segment, label: &str, vb: Option<ViewBox>) -> LabelPosition {
    let dx = (seg.x2 - seg.x1).abs();
    let dy = (seg.y2 - seg.y1).abs();
    let est_width = label.chars().count() as f64 * 6.5;

    if dy > dx {
        let right_edge = vb.map_or(f64::INFINITY, |b| b.right - 12.0);
        if seg.mid_x + 8.0 + est_width > right_edge {
            LabelPosition {
                x: (seg.mid_x - 8.0).round() as i32,
                y: (seg.mid_y + 3.0).round() as i32,
                anchor: "end",
            }
        } else {
            LabelPosition {
                x: (seg.mid_x + 8.0).round() as i32,
                y: (seg.mid_y + 3.0).round() as i32,
                anchor: "start",
            }
        }
    } else {
        let top_limit = vb.map_or(f64::NEG_INFINITY, |b| b.top + 12.0);
        let y = if seg.mid_y - 6.0 < top_limit {
            seg.mid_y + 14.0
        } else {
            seg.mid_y - 6.0
        };
        let (x, anchor) = clamp_horizontal(seg.mid_x, est_width, vb);
        LabelPosition {
            x: x.round() as i32,
            y: y.round() as i32,
            anchor,
        }
    }
}

fn clamp_horizontal(mid_x: f64, est_width: f64, vb: Option<ViewBox>) -> (f64, &'static str) {
    let Some(b) = vb else {
        return (mid_x, "middle");
    };
    if mid_x - est_width / 2.0 < b.left + 4.0 {
        (b.left + 4.0, "start")
    } else if mid_x + est_width / 2.0 > b.right - 4.0 {
        (b.right - 4.0, "end")
    } else {
        (mid_x, "middle")
    }
}

fn parse_view_box(view_box: &str) -> Option<ViewBox> {
    let nums: Vec<f64> = view_box
        .split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty())
        .filter_map(|s| s.parse::<f64>().ok())
        .collect();
    if nums.len() == 4 {
        Some(ViewBox {
            left: nums[0],
            top: nums[1],
            right: nums[0] + nums[2],
        })
    } else {
        None
    }
}

enum Token {
    Cmd(char),
    Num(f64),
}

fn tokenize_path(d: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let bytes = d.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        let ch = bytes[i] as char;
        if ch.is_ascii_alphabetic() {
            tokens.push(Token::Cmd(ch));
            i += 1;
        } else if ch.is_ascii_digit() || ch == '-' || ch == '+' || ch == '.' {
            let start = i;
            i += 1;
            while i < bytes.len() {
                let c = bytes[i] as char;
                if c.is_ascii_digit() || c == '.' {
                    i += 1;
                } else {
                    break;
                }
            }
            if let Ok(n) = d[start..i].parse::<f64>() {
                tokens.push(Token::Num(n));
            }
        } else {
            i += 1;
        }
    }
    tokens
}

fn next_num(tokens: &[Token], idx: &mut usize) -> Option<f64> {
    match tokens.get(*idx) {
        Some(Token::Num(n)) => {
            *idx += 1;
            Some(*n)
        }
        _ => None,
    }
}

fn next_pair(tokens: &[Token], idx: &mut usize) -> Option<(f64, f64)> {
    let x = next_num(tokens, idx)?;
    let y = next_num(tokens, idx)?;
    Some((x, y))
}

struct PathCursor {
    x: f64,
    y: f64,
    cmd: char,
    idx: usize,
}

impl PathCursor {
    fn step(&mut self, tokens: &[Token], segments: &mut Vec<Segment>) -> bool {
        if let Some(Token::Cmd(c)) = tokens.get(self.idx) {
            self.cmd = *c;
            self.idx += 1;
        }
        match self.cmd {
            'M' | 'm' => self.step_move(tokens),
            'L' | 'l' => self.step_line(tokens, segments),
            'H' | 'h' => self.step_horiz(tokens, segments),
            'V' | 'v' => self.step_vert(tokens, segments),
            'Q' | 'q' => self.step_quad(tokens, segments),
            'C' | 'c' => self.step_cubic(tokens, segments),
            _ => next_num(tokens, &mut self.idx).is_some(),
        }
    }

    fn step_move(&mut self, tokens: &[Token]) -> bool {
        let Some((x, y)) = next_pair(tokens, &mut self.idx) else {
            return false;
        };
        if self.cmd == 'm' {
            self.x += x;
            self.y += y;
            self.cmd = 'l';
        } else {
            self.x = x;
            self.y = y;
            self.cmd = 'L';
        }
        true
    }

    fn step_line(&mut self, tokens: &[Token], segments: &mut Vec<Segment>) -> bool {
        let Some((x, y)) = next_pair(tokens, &mut self.idx) else {
            return false;
        };
        let (nx, ny) = if self.cmd == 'l' {
            (self.x + x, self.y + y)
        } else {
            (x, y)
        };
        segments.push(Segment::straight(self.x, self.y, nx, ny));
        self.x = nx;
        self.y = ny;
        true
    }

    fn step_horiz(&mut self, tokens: &[Token], segments: &mut Vec<Segment>) -> bool {
        let Some(x) = next_num(tokens, &mut self.idx) else {
            return false;
        };
        let nx = if self.cmd == 'h' { self.x + x } else { x };
        segments.push(Segment::straight(self.x, self.y, nx, self.y));
        self.x = nx;
        true
    }

    fn step_vert(&mut self, tokens: &[Token], segments: &mut Vec<Segment>) -> bool {
        let Some(y) = next_num(tokens, &mut self.idx) else {
            return false;
        };
        let ny = if self.cmd == 'v' { self.y + y } else { y };
        segments.push(Segment::straight(self.x, self.y, self.x, ny));
        self.y = ny;
        true
    }

    fn step_quad(&mut self, tokens: &[Token], segments: &mut Vec<Segment>) -> bool {
        let Some((cx, cy)) = next_pair(tokens, &mut self.idx) else {
            return false;
        };
        let Some((x, y)) = next_pair(tokens, &mut self.idx) else {
            return false;
        };
        let (cx, cy, nx, ny) = if self.cmd == 'q' {
            (self.x + cx, self.y + cy, self.x + x, self.y + y)
        } else {
            (cx, cy, x, y)
        };
        segments.push(Segment {
            x1: self.x,
            y1: self.y,
            x2: nx,
            y2: ny,
            mid_x: 0.25 * self.x + 0.5 * cx + 0.25 * nx,
            mid_y: 0.25 * self.y + 0.5 * cy + 0.25 * ny,
        });
        self.x = nx;
        self.y = ny;
        true
    }

    fn step_cubic(&mut self, tokens: &[Token], segments: &mut Vec<Segment>) -> bool {
        let Some((c1x, c1y)) = next_pair(tokens, &mut self.idx) else {
            return false;
        };
        let Some((c2x, c2y)) = next_pair(tokens, &mut self.idx) else {
            return false;
        };
        let Some((x, y)) = next_pair(tokens, &mut self.idx) else {
            return false;
        };
        let (c1x, c1y, c2x, c2y, nx, ny) = if self.cmd == 'c' {
            (
                self.x + c1x,
                self.y + c1y,
                self.x + c2x,
                self.y + c2y,
                self.x + x,
                self.y + y,
            )
        } else {
            (c1x, c1y, c2x, c2y, x, y)
        };
        segments.push(Segment {
            x1: self.x,
            y1: self.y,
            x2: nx,
            y2: ny,
            mid_x: 0.125 * self.x + 0.375 * c1x + 0.375 * c2x + 0.125 * nx,
            mid_y: 0.125 * self.y + 0.375 * c1y + 0.375 * c2y + 0.125 * ny,
        });
        self.x = nx;
        self.y = ny;
        true
    }
}

fn parse_segments(d: &str) -> Vec<Segment> {
    let tokens = tokenize_path(d);
    let mut segments = Vec::new();
    let mut cursor = PathCursor {
        x: 0.0,
        y: 0.0,
        cmd: '\0',
        idx: 0,
    };
    while cursor.idx < tokens.len() && cursor.step(&tokens, &mut segments) {}
    segments
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn places_vertical_edge_label_to_the_right_of_midpoint() {
        let pos = edge_label_position("M320,124 L320,150", "when ready", "0 0 640 470");
        assert_eq!(pos.x, 328);
        assert_eq!(pos.y, 140);
        assert_eq!(pos.anchor, "start");
    }

    #[test]
    fn places_horizontal_edge_label_centered_above_midpoint() {
        let pos = edge_label_position("M278,246 L200,246", "no", "0 0 640 470");
        assert_eq!(pos.x, 239);
        assert_eq!(pos.y, 240);
        assert_eq!(pos.anchor, "middle");
    }

    #[test]
    fn picks_longest_segment_for_polyline_edges() {
        let pos = edge_label_position(
            "M110,270 L110,332 L220,332",
            "deferred branch",
            "0 0 640 470",
        );
        assert_eq!(pos.x, 165);
        assert_eq!(pos.y, 326);
        assert_eq!(pos.anchor, "middle");
    }

    #[test]
    fn flips_vertical_label_left_when_near_right_edge_of_viewbox() {
        let pos = edge_label_position(
            "M440,414 L610,414 L610,32 L420,32",
            "return to start",
            "0 0 640 470",
        );
        assert_eq!(pos.x, 602);
        assert_eq!(pos.y, 226);
        assert_eq!(pos.anchor, "end");
    }

    #[test]
    fn places_quadratic_curve_label_on_curve_midpoint() {
        let pos = edge_label_position("M0,100 Q100,0 200,100", "arc", "0 0 300 200");
        assert_eq!(pos.x, 100);
        assert_eq!(pos.y, 44);
        assert_eq!(pos.anchor, "middle");
    }
}
