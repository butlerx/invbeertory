import React from 'react';
import { Index } from 'elasticlunr';
import { Link } from 'gatsby';
import { makeUrl } from '../utils';
import { Beer } from '../types';
import { search as searchStyles, searchResults, searchBox } from './styles/search.module.scss';
import { path } from './styles/layout.module.scss';

interface Props {
  searchIndex: Record<string, string>;
}
type SetQuery = React.Dispatch<React.SetStateAction<string>>;
type SetResults = React.Dispatch<React.SetStateAction<Beer[]>>;

const search =
  (setQuery: SetQuery, setResults: SetResults, index: Index) =>
  (evt: React.ChangeEvent<HTMLInputElement>): void => {
    const query = evt.target.value;
    setQuery(query);
    setResults(
      index
        .search(query, { expand: true })
        .map(({ ref }: { ref: string }) => index.documentStore.getDoc(ref)),
    );
  };

export function Search({ searchIndex }: Props): React.ReactElement {
  const [query, setQuery]: [string, SetQuery] = React.useState('');
  const [results, setResults]: [Beer[], SetResults] = React.useState([]);
  const index = Index.load(searchIndex);
  return (
    <div className={searchStyles}>
      <input
        type="text"
        name="search"
        aria-label="Search"
        placeholder="What beer you looking for?"
        className={searchBox}
        value={query}
        onChange={search(setQuery, setResults, index)}
      />
      <ul className={searchResults}>
        {results.map((page: Beer) => (
          <li key={page.id}>
            <Link to={makeUrl(page)} className={path}>
              {page.name} - {page.year} ({page.abv}%)
            </Link>
          </li>
        ))}
      </ul>
    </div>
  );
}
