import { writable } from 'svelte/store';

export interface MediaItem {
  id: string;
  title: string;
  artist: string;
  album: string;
  duration: string;
  poster?: string;
  backdrop?: string;
  year: string;
  rating: number;
  genres: string[];
  synopsis: string;
  director: string;
  starring: string;
  type: 'video' | 'audio' | 'unknown';
  subtitle: string;
}

const namedItems: MediaItem[] = [
  {
    id: "1", title: "dropped_video.mp4", artist: "Unknown Artist", album: "Unknown Album",
    duration: "1:23:45", year: "2024", rating: 0, genres: [],
    synopsis: "No synopsis available.", director: "Unknown", starring: "Unknown",
    type: "video", subtitle: "None"
  },
  {
    id: "2", title: "Blue Lock-S2E1-480P", artist: "Unknown Artist", album: "Blue Lock",
    duration: "24:00", year: "2024", rating: 4.8,
    genres: ["Anime", "Sports"], synopsis: "The second selection begins.",
    director: "Shunsuke Ishikawa", starring: "Nobunaga Shimazaki, Kazuki Ura",
    type: "video", subtitle: "English"
  },
  {
    id: "3", title: "BLUE LOCK THE MOVIE -EPISODE NAGI-", artist: "Unknown Artist",
    album: "Blue Lock", duration: "1:30:00", year: "2024",
    rating: 4.9, genres: ["Anime", "Sports", "Movie"],
    synopsis: "Nagi Seishiro's journey to Blue Lock.", director: "Shunsuke Ishikawa",
    starring: "Nobunaga Shimazaki, Yuma Uchida", type: "video", subtitle: "English"
  },
  {
    id: "4", title: "Guardians of the galaxy", artist: "Marvel", album: "Guardians Vol. 3",
    duration: "2:30:00", year: "2023", rating: 4.5,
    genres: ["Action", "Sci-Fi", "Comedy"],
    synopsis: "Still reeling from the loss of Gamora, Peter Quill rallies his team.",
    director: "James Gunn", starring: "Chris Pratt, Zoe Saldana",
    type: "video", subtitle: "English"
  },
  {
    id: "5", title: "Fi Kan We Kan (feat. Rema)", artist: "BNXN", album: "Unknown Album",
    duration: "2:39",
    poster: "https://i.scdn.co/image/ab67616d0000b273d46a8d799059f0861113b244",
    backdrop: "https://i.scdn.co/image/ab67616d0000b273d46a8d799059f0861113b244",
    year: "2024", rating: 5.0, genres: ["Afrobeats", "Pop"],
    synopsis: "A hit collaboration between BNXN and Rema.",
    director: "Unknown", starring: "BNXN, Rema", type: "audio", subtitle: "None"
  },
  {
    id: "6", title: "Addicted | Xclusiveloaded.com", artist: "Unknown Artist",
    album: "Unknown Album", duration: "3:15", year: "2024",
    rating: 0, genres: [], synopsis: "Audio track.", director: "Unknown",
    starring: "Unknown", type: "audio", subtitle: "None"
  },
  {
    id: "7", title: "Are You There || TrendyBeatz.com", artist: "Unknown Artist",
    album: "Unknown Album", duration: "4:00", year: "2024",
    rating: 0, genres: [], synopsis: "Audio track.", director: "Unknown",
    starring: "Unknown", type: "audio", subtitle: "None"
  },
];

// Generate 17 more placeholder items to reach 24 total
const placeholderItems: MediaItem[] = Array.from({ length: 17 }, (_, i) => ({
  id: String(i + 8),
  title: `Placeholder File ${i + 8}`,
  artist: "Unknown Artist",
  album: "Unknown Album",
  duration: "0:00",
  year: "2024",
  rating: 0,
  genres: [] as string[],
  synopsis: "No synopsis available.",
  director: "Unknown",
  starring: "Unknown",
  type: (i % 3 === 0 ? "audio" : "video") as 'video' | 'audio' | 'unknown',
  subtitle: "None"
}));

export const mockMediaItems: MediaItem[] = [...namedItems, ...placeholderItems];

export const mediaItems = writable<MediaItem[]>(mockMediaItems);
export const selectedMediaId = writable<string | null>(null);
