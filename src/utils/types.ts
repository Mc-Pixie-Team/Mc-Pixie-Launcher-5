enum Provider {
  Modrinth = "Modrinth",
  Curseforge = "Curseforge",
  Pixie = "Pixie",
}

interface FirstRequestData {
  name: string;
  iconUri: string;
  description?: string;
  author?: string;
  categories?: string[];
  downloads?: number;
  provider: Provider;
  rawProjectData: any;
}






export { Provider };
export type { FirstRequestData };
