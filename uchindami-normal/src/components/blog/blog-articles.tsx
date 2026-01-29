import { Button } from "@/components/ui/button.tsx";
import { Clock, User } from "lucide-react";
import React from "react";

const BlogArticleComponent = ({ frontmatter, url }) => {
  const { pubDate, title, description, image, mainTag, author, readTime } =
    frontmatter;

  const formatDate = (date) => {
    return new Date(date).toLocaleDateString("en-GB", {
      day: "numeric",
      month: "long",
      year: "numeric",
    });
  };

  return (
    <article className="rounded-lg shadow-md overflow-hidden">
      <div className="lg:hidden text-sm px-6 pt-6">
        <h2>{formatDate(pubDate)}</h2>
      </div>
      <div className="lg:flex">
        <div className="p-6">
          <a href={url}>
            <h3 className="text-xl font-semibold text-blog-secondary mb-2">
              {title}
            </h3>
          </a>
          <p className="text-gray-700 mb-4 text-pretty">{description}</p>
          <Button variant="link" className="p-0">
            <a href={url} title="Read More" className={"underline"}>
              Read More
            </a>
          </Button>
        </div>
        <div className="flex align-center lg:block justify-center m-2">
          <img
            src={image.url}
            alt="Article image"
            className="w-1/2 md:w-1/3 lg:w-2/3 object-cover"
          />
        </div>
      </div>
      <div className="px-6 py-4 bg-gray-200 flex items-center justify-between">
        <div className="flex items-center space-x-4">
          <span className="inline-block text-pretty lg:bg-gray-200 rounded-full md:p-2 lg:px-3 p-2 text-sm font-semibold text-gray-700">
            {mainTag}
          </span>
          <span className="flex items-center text-sm text-gray-600">
            <User className="hidden lg:block w-4 h-4 mr-1" />
            By {author}
          </span>
        </div>
        <span className="hidden lg:block flex items-center text-sm text-gray-600">
          <h2 className="text-gray-600">{formatDate(pubDate)}</h2>
        </span>
        <span className="flex items-center text-sm text-gray-600">
          <Clock className="w-4 h-4 mr-1" />
          {readTime}m Read
        </span>
      </div>
    </article>
  );
};

export const BlogArticles = ({ allPosts }) => {
  return (
    <div className="lg:h-[calc(100vh-200px)] lg:overflow-y-auto blog-scrollbar">
      <div className="space-y-8 lg:mx-2">
        {allPosts.map((article, index) => (
          <BlogArticleComponent key={index} {...article} />
        ))}
      </div>
    </div>
  );
};
