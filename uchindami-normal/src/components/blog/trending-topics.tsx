import { Button } from "@/components/ui/button";
import React from "react";

interface TrendingTopic {
  name: string;
}

const trendingTopics: TrendingTopic[] = [
  { name: "Design Thinking" },
  { name: "Technology" },
  { name: "Web3" },
  { name: "Programming" },
  { name: "AI" },
];

export const TrendingTopics: React.FC = () => (
  <div className="hidden lg:block mb-6  ml-2">
    <h2 className="font-semibold mb-2">Trending Topics</h2>
    <div className="flex gap-2 flex-wrap">
      {trendingTopics.map((topic) => (
        <Button key={topic.name} variant="tag" size="sm">
          {topic.name}
        </Button>
      ))}
    </div>
  </div>
);
