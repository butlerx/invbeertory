export interface Beer {
  id: string;
  name: string;
  brewery: string;
  year: number;
  abv: number;
  style: string;
  size: number;
  drunk: boolean;
  stock: number;
  purchased: number;
  ibu?: number;
  collaborators?: string;
  barrelAged: boolean;
  barrelType?: string;
  brewedWith?: string;
}
