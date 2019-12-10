export interface Beer {
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
  collaborator?: string;
  barrelAged: boolean;
  barrelType?: string;
  brewedWith?: string;
}

export interface StockProps {
  data: {
    allGoogleSheetInventoryRow: {
      nodes: Beer[];
    };
  };
}
