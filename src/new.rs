struct Rect {
	point: Point,
	size: Size
}
impl Rect {
	fn new() -> Self {
		Rect {
			point:Point::new(),
			size:Size::new()
		}
	}
}

struct Point {
	x: f64,
	y: f64
}
impl Point {
	fn new() -> Self {
		Point { x: 0.0, y: 0.0 }
	}
}

struct Size {
	width: f64,
	height: f64
}
impl Size {
	fn new() -> Self {
		Size { width: 0.0, height: 0.0 }
	}
		fn from_width_height(w:f64, h:f64) -> Self {
		Size { width: w, height: h }
	}
}
trait MeasureArrange {
	fn get_desired_size(&self) -> &Size;
	fn measure(&self, _available_size: Size) -> Size;
	fn arrange(&self, _final_size: Size) -> Size;
}
trait UIElement {
	fn get_children(&self) -> &Vec<Box<UIElement>>;
	fn render(&self, &renderer: Renderer);
}
struct Panel {
	children: Vec<Box<UIElement>>,
	position: Point,
	desired_size: Size,
	actual_size: Size
}
impl UIElement for Panel {
	fn get_children(&self) -> &Vec<Box<UIElement>> {
		return &self.children;
	}

	fn render(&self, &renderer: Renderer) {
		let rect = Rect::new();
		renderer.draw_rectange(rect);
	}
}
impl Panel {
	fn new() -> Self {
		Panel {
			children: Vec::new(),
			position: Point::new(),
			desired_size: Size::new(),
			actual_size: Size::new()
		}
	}

	fn add_child(&mut self, c:impl UIElement + 'static) {
	    self.children.push(Box::new(c));
	}

	fn layout(&self, _available_size: Size)
	{
		for child in self.children {
			self.desired_size = self.measure(_available_size);
			self.actual_size = self.arrange(self.desired_size);
		}
	}
}

impl MeasureArrange for Panel {
	fn get_desired_size(&self) -> &Size
	{
		return &self.desired_size;
	}
	fn measure(&self, _available_size: Size) -> Size {
		return _available_size;
	}
	fn arrange(&self, _final_size: Size) -> Size {
		return _final_size;
	}
}

trait Renderer{
	fn draw_rectange(&self, r:Rect);

	// Future
	// fn draw_ellipse(&self);
	// fn draw_triangle(&self);
}

struct SillyConsoleRenderer {

}

impl SillyConsoleRenderer {
	fn new() -> Self {
		SillyConsoleRenderer {
		}
	}
}

impl Renderer for SillyConsoleRenderer {
	fn draw_rectange(&self, r: Rect) {
		println!("look ma! no hands!");
		println!("{:?}", r.point.x );
	}
}

fn main() {
	let mut root = Panel::new();
	root.children.push(Box::new(Panel::new()));
	root.add_child(Panel::new());
	root.add_child(Panel::new());
	root.add_child(Panel::new());

	let available_area = Size::from_width_height(1000.0, 800.0);
	root.layout(available_area);

	let silly_console_renderer = SillyConsoleRenderer::new();
	root.render(silly_console_renderer);
}
