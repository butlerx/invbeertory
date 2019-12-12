import React, { FC, useState, ChangeEvent } from 'react';
import { Index } from 'elasticlunr';
import { Link } from 'gatsby';
import { makeUrl } from '../utils';
import { Beer } from '../types';
import { search as searchStyles, searchResults, searchBox } from './styles/search.module.scss';
import { path } from './styles/layout.module.scss';

interface Props {
  searchIndex: object;
}

const search = (setQuery: Function, setResults: Function, index: Index) => (
  evt: ChangeEvent<HTMLInputElement>,
): void => {
  const query = evt.target.value;
  setQuery(query);
  setResults(
    index
      .search(query, { expand: true })
      .map(({ ref }: { ref: string }) => index.documentStore.getDoc(ref)),
  );
};

export const Search: FC<Props> = ({ searchIndex }) => {
  const [query, setQuery] = useState('');
  const [results, setResults] = useState([]);
  const index = Index.load(searchIndex);
  return (
    <div className={searchStyles}>
      <input
        type="text"
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
};
