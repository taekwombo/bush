export class Color {
    public static White: Color = new Color(255, 255, 255, 255);
    public static Red: Color = new Color(255, 0, 0, 255);
    public static Black: Color = new Color(0, 0, 0, 255);
    public static Blue: Color = new Color(0, 0, 255, 255);
    public static Unpainted: Color = new Color();

    public r: number;
    public g: number;
    public b: number;
    public a: number;

    public constructor(r?: number, g?: number, b?: number, a?: number) {
        this.r = r || 0;
        this.g = g || 0;
        this.b = b || 0;
        this.a = a || 0;
    }
}
