export interface UrlData {
  id?: number;
  short_code: string;
  original_url: string;
  short_url: string;
  click_count: number;
  created_at: string;
  expires_at?: string | null;
}

export interface DashboardStats {
  total_links: number;
  total_clicks: number;
  top_link: UrlData | null;
}

export interface StatEntry {
  id: number;
  url_id: number;
  user_agent: string | null;
  ip_address: string | null;
  referrer: string | null;
  country_code: string | null;
  created_at: string;
}

export interface PaginationMeta {
  page: number;
  per_page: number;
  total_records: number;
  total_pages: number;
}

export interface UrlStatsResponse {
  short_code: string;
  short_url: string;
  original_url: string;
  total_clicks: number;
  created_at: string;
  expires_at: string | null;
  pagination?: PaginationMeta;
  stats: StatEntry[];
}
