import { writable } from 'svelte/store';

export interface MediaItem {
  id: string;
  title: string;
  artist: string;
  album: string;
  duration: string;
  poster: string;
  backdrop: string;
  year: string;
  rating: number;
  genres: string[];
  synopsis: string;
  director: string;
  starring: string;
  subtitle: string;
}

export const mockMediaItems: MediaItem[] = [
  {
    id: "1",
    title: "dropped_video.mp4",
    artist: "Unknown Artist",
    album: "Unknown Album",
    duration: "1:23:45",
    poster: "/flux.png",
    backdrop: "/flux.png",
    year: "2024",
    rating: 0,
    genres: [],
    synopsis: "No synopsis available.",
    director: "Unknown",
    starring: "Unknown",
    subtitle: "None"
  },
  {
    id: "2",
    title: "Blue Lock-S2E1-480P",
    artist: "Unknown Artist",
    album: "Blue Lock",
    duration: "24:00",
    poster: "/flux.png",
    backdrop: "/flux.png",
    year: "2024",
    rating: 4.8,
    genres: ["Anime", "Sports"],
    synopsis: "The second selection begins.",
    director: "Shunsuke Ishikawa",
    starring: "Nobunaga Shimazaki, Kazuki Ura",
    subtitle: "English"
  },
  {
    id: "3",
    title: "BLUE LOCK THE MOVIE -EPISODE NAGI-",
    artist: "Unknown Artist",
    album: "Blue Lock",
    duration: "1:30:00",
    poster: "/flux.png",
    backdrop: "/flux.png",
    year: "2024",
    rating: 4.9,
    genres: ["Anime", "Sports", "Movie"],
    synopsis: "Nagi Seishiro's journey to Blue Lock.",
    director: "Shunsuke Ishikawa",
    starring: "Nobunaga Shimazaki, Yuma Uchida",
    subtitle: "English"
  },
  {
    id: "4",
    title: "Guardians of the galaxy",
    artist: "Marvel",
    album: "Guardians Vol. 3",
    duration: "2:30:00",
    poster: "/flux.png",
    backdrop: "/flux.png",
    year: "2023",
    rating: 4.5,
    genres: ["Action", "Sci-Fi", "Comedy"],
    synopsis: "Still reeling from the loss of Gamora, Peter Quill rallies his team to defend the universe and one of their own.",
    director: "James Gunn",
    starring: "Chris Pratt, Zoe Saldana",
    subtitle: "English"
  },
  {
    id: "5",
    title: "Fi Kan We Kan (feat. Rema)",
    artist: "BNXN",
    album: "Unknown Album",
    duration: "2:39",
    poster: "https://i.scdn.co/image/ab67616d0000b273d46a8d799059f0861113b244", // Using a placeholder that will be replaced or look generic
    backdrop: "https://i.scdn.co/image/ab67616d0000b273d46a8d799059f0861113b244",
    year: "2024",
    rating: 5.0,
    genres: ["Afrobeats", "Pop"],
    synopsis: "A hit collaboration between BNXN and Rema.",
    director: "Unknown",
    starring: "BNXN, Rema",
    subtitle: "None"
  },
  {
    id: "6",
    title: "Addicted | Xclusiveloaded.com",
    artist: "Unknown Artist",
    album: "Unknown Album",
    duration: "3:15",
    poster: "/flux.png",
    backdrop: "/flux.png",
    year: "2024",
    rating: 0,
    genres: [],
    synopsis: "Audio track.",
    director: "Unknown",
    starring: "Unknown",
    subtitle: "None"
  },
  {
    id: "7",
    title: "Are You There || TrendyBeatz.com",
    artist: "Unknown Artist",
    album: "Unknown Album",
    duration: "4:00",
    poster: "/flux.png",
    backdrop: "/flux.png",
    year: "2024",
    rating: 0,
    genres: [],
    synopsis: "Audio track.",
    director: "Unknown",
    starring: "Unknown",
    subtitle: "None"
  }
];

export const mediaItems = writable<MediaItem[]>(mockMediaItems);
export const selectedMediaId = writable<string | null>("5");
