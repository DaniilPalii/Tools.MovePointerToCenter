var screen = Screen.PrimaryScreen!.Bounds;

Cursor.Position = new(
	x: screen.X + (screen.Width / 2),
	y: screen.Y + (screen.Height / 2));
