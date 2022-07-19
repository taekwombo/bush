export class Color {
    public static Aqua = new Color(0, 255, 255, 255);
    public static Black = new Color(0, 0, 0, 255);
    public static Blue = new Color(0, 0, 255, 255);
    public static Fuchsia = new Color(255, 0, 255, 255);
    public static Gray = new Color(128, 128, 128, 255);	
    public static Green = new Color(0, 128, 0, 255);	
    public static Lime = new Color(0, 255, 0, 255);	
    public static Maroon = new Color(128, 0, 0, 255);
    public static Navy = new Color(0, 0, 128, 255);	
    public static Olive = new Color(128, 128, 0, 255);	
    public static Purple = new Color(128, 0, 128, 255);	
    public static Red = new Color(255, 0, 0, 255);
    public static Silver = new Color(192, 192, 192, 255);	
    public static Teal = new Color(0, 128, 128, 255);	
    public static White = new Color(255, 255, 255, 255);	
    public static Yellow = new Color(255, 255, 0, 255);
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

    public eq(other: Color): boolean {
        return this.r === other.r && this.g === other.g && this.b === other.b && this.a === other.a;
    }
}
