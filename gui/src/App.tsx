import React, { useState, useEffect } from "react";
import { ArrowLeftIcon, ArrowRightIcon } from "@heroicons/react/solid"
import { RecoilRoot, atom, useRecoilState, useRecoilValue, selector } from "recoil";
import { Listbox } from "@headlessui/react";
import debounce_async from "debounce-promise";
import { invoke } from "@tauri-apps/api/tauri";
import {
  HashRouter as Router,
  Link,
  NavLink,
  Route,
  Switch,
  useHistory,
  useParams,
} from "react-router-dom";

const detailState = atom<Details | null>({ key: "detailState", default: null });

type ItemProps = {
  item: SearchResult;
  onClick: (item: SearchResult) => void;
};
function Item(props: ItemProps) {
  return (
    <div
      className="h-72 flex flex-col rounded shadow-md cursor-pointer"
      onClick={(_) => props.onClick(props.item)}
    >
      <img className="h-60" src={props.item.cover_image_url} />
      <span className="text-center pt-2">{props.item.title}</span>
    </div>
  );
}

const search = debounce_async(async (text: string): Promise<SearchResult[]> => {
  if (text.length >= 3) {
    return await invoke("gogoanime_search", { text });
  }

  return [];
}, 300);

type SearchResult = {
  cover_image_url: string;
  title: string;
  slug: string;
};

function SearchView() {
  const [result, setResult] = useState<SearchResult[]>([]);
  const [text, setText] = useState("");
  const history = useHistory();

  useEffect(() => {
    const res = search(text);

    if (res) {
      res.then(res => {
        setResult(res)
      });
    }
  }, [text]);

  return (
    <div className="flex flex-col items-center h-full">
      <input
        className="bg-gray-100 rounded-lgm p-2 w-1/2"
        placeholder="Search..."
        onChange={(ev) => setText(ev.target.value)}
        value={text}
      />
      <div className="mt-10 w-full h-full grid grid-cols-8 gap-6">
        {result.map((a, i) => (
          <Item
            key={i}
            item={a}
            onClick={(item) => {
              console.log(item);
              history.push(`/anime/${item.slug}`);
            }}
          />
        ))}
      </div>
    </div>
  );
}

// #[derive(Debug, serde::Serialize)]
// pub struct Anime {
//     pub slug: String,
//     pub title: String,
// }

type Details = {
  id: number;
  cover_image_url: string;
  summary: string;
  title: string;
  genres: string[];
  release_year: number;
  default_episode: number;
  episode_count: number;
};

type Episode = {
  providers: [string, string][];
};

type EpisodeGridProps = {
  episodeCount: number;
  currentEpisode?: number;
  currentEpisodeClassName?: string;
  onClick: (i: number) => void;
};
function EpisodeGrid(props: EpisodeGridProps) {
  return (
    <div className="m-10 grid grid-cols-10 gap-3">
      {Array(props.episodeCount)
        .fill(0)
        .map((_, i) => (
          <div
            key={i}
            onClick={(_) => props.onClick(i + 1)}
            className={
              "border text-center p-2 cursor-pointer hover:bg-gray-100 " +
              (props.currentEpisodeClassName &&
              props.currentEpisode &&
              i + 1 === props.currentEpisode
                ? props.currentEpisodeClassName
                : "")
            }
          >
            {i + 1}
          </div>
        ))}
    </div>
  );
}

function DetailsView() {
  const { slug } = useParams<{ slug: string }>();
  const [details, setDetails] = useRecoilState(detailState);
  const history = useHistory();

  useEffect(() => {
    invoke<Details>("gogoanime_details", { slug }).then((x) => {
      setDetails(x);
    });
  }, [slug]);

  if (!details) {
    return <div></div>;
  }

  return (
    <div>
      <div className="flex flex-col">
        <div className="flex">
          <img className="h-96 shadow mr-10" src={details.cover_image_url} />
          <div className="flex flex-col">
            <h1 className="text-3xl mb-10">{details.title}</h1>
            <p>{details.summary}</p>
          </div>
        </div>
      </div>
      <EpisodeGrid
        episodeCount={details.episode_count}
        onClick={(i) => history.push(`/anime/${slug}/${i}`)}
      />
    </div>
  );
}

function EpisodesView() {
  const params = useParams<{ slug: string; episode: string }>();
  const slug = params.slug;
  const history = useHistory();
  const episode_number = Number(params.episode);
  const [details, setDetails] = useRecoilState(detailState);
  const [provider, setProvider] = useState<[string, string] | null>(null);
  const [episode, setEpisode] = useState<Episode | null>(null);

  useEffect(() => {
    invoke<Episode>("gogoanime_episode", {
      slug,
      episode: episode_number,
    }).then((x) => {
      console.log(x);
      setEpisode(x);
    });
  }, [slug, episode_number]);

  useEffect(() => {
    if (episode) {
      setProvider(provider 
        ? episode.providers.find(p => p[0] == provider[0]) || episode.providers[0]
        : episode.providers[0]);
    }
  }, [episode]);

  if (!episode || !provider || !details) {
    return <div></div>;
  }

  const requiresEmbed = ![].some((x) => x === provider[0]);

  return (
    <div className="flex flex-col items-center">
      <div style={{ maxWidth: "1280px"}}>
        <div className="flex mb-5">
          <h1 className="text-3xl inline-block mr-auto"><Link to={`/anime/${slug}`}>{details.title}</Link> - Episode {episode_number}</h1>
          <div>
            <Listbox value={provider} onChange={setProvider}>
              <Listbox.Button className="border px-3 py-1 hover:bg-gray-100 mb-2">
                {provider[0]}
              </Listbox.Button>
              <Listbox.Options className="border w-32 absolute">
                {episode.providers.map((p, i) => (
                  <Listbox.Option
                    className="px-3 py-1 bg-white hover:bg-gray-100 cursor-pointer"
                    key={i}
                    value={p}
                  >
                    {p[0]}
                  </Listbox.Option>
                ))}
              </Listbox.Options>
            </Listbox>
          </div>
        </div>
        {requiresEmbed ? (
          <iframe
            allowFullScreen
            height="720"
            width="1280"
            src={provider[1]}
          ></iframe>
        ) : (
          <video height="720" width="1280">
            <source src={provider[1]} type="video/mp4" />
          </video>
        )}
        <div className="flex justify-end px-5 pt-2 w-full">
          <Link className="mr-auto flex items-center" to={`/anime/${slug}/${episode_number - 1}`}> <ArrowLeftIcon className="h-4 mr-2"/> Previous </Link>
          <Link className="flex items-center" to={`/anime/${slug}/${episode_number + 1}`}> Next <ArrowRightIcon className="h-4 ml-2"/> </Link>
        </div>
        <EpisodeGrid
          currentEpisode={episode_number}
          currentEpisodeClassName="bg-gray-100"
          episodeCount={details.episode_count}
          onClick={episode => history.push(`/anime/${slug}/${episode}`)}
        />
      </div>
    </div>
  );
}

function App() {
  return (
    <RecoilRoot>
      <Router>
        <div className="h-full flex flex-col items-center">
          <nav className="flex shadow w-full">
            <NavLink
              exact
              className="p-4 hover:bg-gray-100 cursor-pointer"
              activeClassName="font-bold"
              to="/"
            >
              Home
            </NavLink>
          </nav>
          <div className="container pt-10 h-full">
            <Switch>
              <Route exact path="/">
                <SearchView />
              </Route>
              <Route exact path="/anime/:slug">
                <DetailsView />
              </Route>
              <Route exact path="/anime/:slug/:episode">
                <EpisodesView />
              </Route>
            </Switch>
          </div>
        </div>
      </Router>
    </RecoilRoot>
  );
}

export default App;
