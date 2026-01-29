"use client";

import { useState, useEffect, useRef } from "react";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Search, X } from "lucide-react";

// Custom hook for search functionality
function useSearch(items, searchTerm) {
  const [results, setResults] = useState([]);

  useEffect(() => {
    const filteredResults = items.filter(
      (item) =>
        item.frontmatter.title
          .toLowerCase()
          .includes(searchTerm.toLowerCase()) ||
        item.frontmatter.description
          .toLowerCase()
          .includes(searchTerm.toLowerCase()) ||
        item.frontmatter.author
          .toLowerCase()
          .includes(searchTerm.toLowerCase()),
    );
    setResults(filteredResults);
  }, [items, searchTerm]);

  return results;
}

export default function searchBox({ articleData }) {
  const [searchTerm, setSearchTerm] = useState("");
  const [isModelOpen, setIsModelOpen] = useState(false);
  const results = useSearch(articleData, searchTerm);
  const wrapperRef = useRef(null);

  useEffect(() => {
    function handleClickOutside(event) {
      if (wrapperRef.current && !wrapperRef.current.contains(event.target)) {
        setIsModelOpen(false);
      }
    }

    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, [wrapperRef]);

  useEffect(() => {
    if (searchTerm) {
      setIsModelOpen(true);
    }
  }, [searchTerm]);

  const handleClearSearch = () => {
    setSearchTerm("");
    setIsModelOpen(false);
  };

  return (
    <div className="w-full max-w-md mx-auto" ref={wrapperRef}>
      <div className="relative">
        <Input
          type="text"
          placeholder="Search blog posts..."
          value={searchTerm}
          onChange={(e) => setSearchTerm(e.target.value)}
          className="pr-10"
        />
        <Button
          size="icon"
          variant="ghost"
          className="absolute right-0 top-0 h-full"
          onClick={isModelOpen ? handleClearSearch : undefined}
        >
          {isModelOpen ? (
            <X className="h-4 w-4" />
          ) : (
            <Search className="h-4 w-4" />
          )}
          <span className="sr-only">
            {isModelOpen ? "Clear search" : "Search"}
          </span>
        </Button>
      </div>
      {isModelOpen && (
        <div className="absolute bg-white mt-1 z-10 w-1/2">
          <ScrollArea className="h-72 w-full rounded-md border bg-background shadow-md">
            <div className="p-4">
              <h4 className="mb-4 text-sm font-medium leading-none">
                Search Results
              </h4>
              {results.length > 0 ? (
                <ul className="space-y-4">
                  {results.map((item, index) => (
                    <li key={index} className="border-b pb-2 last:border-b-0">
                      <a href={item.url}>
                        <h5 className="font-semibold text-sm">
                          {item.frontmatter.title}
                        </h5>
                      </a>
                      <p className="text-xs truncate mt-1 ">
                        {item.frontmatter.description}
                      </p>
                      <div className="flex justify-between items-center mt-2">
                        <span className="text-xs text-muted-foreground">
                          By {item.frontmatter.author}
                        </span>
                        <span className="text-xs bg-primary/10 text-primary px-2 py-1 rounded-full">
                          {item.frontmatter.mainTag[0]}
                        </span>
                      </div>
                    </li>
                  ))}
                </ul>
              ) : (
                <p className="text-sm text-muted-foreground">
                  No results found
                </p>
              )}
            </div>
          </ScrollArea>
        </div>
      )}
    </div>
  );
}
