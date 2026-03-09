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
