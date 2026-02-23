import React, { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { Link2, ArrowRight, Check, Copy, BarChart3 } from "lucide-react";
import { Button } from "../ui/Button/Button";

export default function UrlForm() {
  const [url, setUrl] = useState("");
  const [isLoading, setIsLoading] = useState(false);
  const [shortenedLink, setShortenedLink] = useState<string | null>(null);
  const [copied, setCopied] = useState(false);

  const handleShorten = async () => {
    setIsLoading(true);
    // Simulación de lógica de backend en Rust
    setTimeout(() => {
      setShortenedLink("short.it/xyz123");
      setIsLoading(false);
    }, 1000);
  };

  const copyToClipboard = () => {
    navigator.clipboard.writeText(shortenedLink || "");
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="w-full max-w-xl">
      <div className="group relative flex items-center rounded-xl border border-border bg-card p-1.5 transition-all focus-within:border-primary/50">
        <Link2 className="ml-3 h-4 w-4 shrink-0 text-muted-foreground" />
        <input
          type="url"
          placeholder="Paste your long URL here..."
          value={url}
          onChange={(e) => setUrl(e.target.value)}
          className="flex-1 bg-transparent px-3 py-2.5 text-sm focus:outline-none"
        />
        <Button
          className="cursor-pointer"
          onClick={handleShorten}
          disabled={isLoading || !url.trim()}
        >
          {isLoading ? (
            "..."
          ) : (
            <>
              <span className="hidden sm:inline">Shorten</span>{" "}
              <ArrowRight className="h-3.5 w-3.5" />
            </>
          )}
        </Button>
      </div>

      <AnimatePresence>
        {shortenedLink && (
          <motion.div
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            className="mt-6 glass rounded-xl p-5 border border-primary/20"
          >
            <div className="flex items-center justify-between gap-4">
              <div className="min-w-0 flex-1">
                <p className="truncate text-sm font-semibold text-primary">
                  {shortenedLink}
                </p>
                <p className="truncate text-xs text-muted-foreground">{url}</p>
              </div>
              <div className="flex gap-2">
                <Button variant="outline" size="sm" onClick={copyToClipboard}>
                  {copied ? (
                    <Check className="h-3.5 w-3.5" />
                  ) : (
                    <Copy className="h-3.5 w-3.5" />
                  )}
                </Button>
                <Button variant="ghost" size="sm">
                  <BarChart3 className="h-3.5 w-3.5" />
                </Button>
              </div>
            </div>
          </motion.div>
        )}
      </AnimatePresence>
    </div>
  );
}
