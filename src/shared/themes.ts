export const lightUIThemes = [
  "light",
  "cupcake",
  "bumblebee",
  "emerald",
  "corporate",
  "retro",
  "cyberpunk",
  "valentine",
  "garden",
  "pastel",
  "fantasy",
  "wireframe",
  "cmyk",
  "autumn",
  "acid",
  "lemonade",
  "nord",
  "caramellatte",
  "silk",
] as const;

export type LightUITheme = (typeof lightUIThemes)[number];

export const darkUIThemes = [
  "dark",
  "synthwave",
  "halloween",
  "forest",
  "aqua",
  "lofi",
  "black",
  "luxury",
  "dracula",
  "business",
  "night",
  "coffee",
  "winter",
  "dim",
  "sunset",
  "abyss",
] as const;

export type DarkUITheme = (typeof darkUIThemes)[number];

export const UIThemes = [...lightUIThemes, ...darkUIThemes] as const;

export type UITheme = (typeof UIThemes)[number];

// TODO: Add support for square assets/textures (wood, marble, etc.)
export type BoardTheme = {
  lightSquare: string;
  darkSquare: string;
  displayCoordinates: boolean;
  pieceSet: "standard"; // TODO: Add support for other piece sets
};

export const BoardThemes: Record<string, BoardTheme> = {
  light: {
    lightSquare: "#f0d9b5",
    darkSquare: "#b58969",
    displayCoordinates: true,
    pieceSet: "standard",
  },
  dark: {
    lightSquare: "#d9dedf",
    darkSquare: "#7a929b",
    displayCoordinates: true,
    pieceSet: "standard",
  },
  // TODO: Add more themes/support for custom themes
} as const;
