export interface Bean {
  name: string;
  process: string;
  region: string;
  country: string;
  variety: string;
  tasting: string;
  altitude: number;
}

export interface Coffee {
  roaster: string;
  name: string;
  roasted: Date;
  size: number;
  stock: number;
  purchased: number;
  beans: Bean[];
}
