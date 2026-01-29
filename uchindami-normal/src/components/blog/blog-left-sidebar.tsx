import { ScrollArea } from "@/components/ui/scroll-area";

export const BlogLeftSidebar = () => {
  const blogTopics = [
    "Company",
    "Design",
    "Technology",
    "Crypto",
    "Artificial Intelligence",
    "Work",
  ];

  const guideAndTools = ["Getting Started", "Documentation", "API Reference"];

  return (
    <>
      <div className="hidden lg:block ">
        <ScrollArea className="p-4">
          <nav>
            <div className="mb-6">
              <h2 className="text-sm font-semibold mb-2 text-blog-primary">
                Blog Topics
              </h2>
              <ul className="space-y-2">
                {blogTopics.map((topic, index) => (
                  <li key={index}>
                    <a
                      href="#"
                      className=" text-sm text-blog-secondary hover:underline"
                    >
                      {topic}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
            <div>
              <h2 className="text-sm font-semibold mb-2 text-blog-primary">
                Guide and Tools
              </h2>
              <ul className="space-y-2">
                {guideAndTools.map((item, index) => (
                  <li key={index}>
                    <a
                      href="#"
                      className=" text-sm text-blog-secondary hover:underline"
                    >
                      {item}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          </nav>
        </ScrollArea>
      </div>
      <div />
    </>
  );
};
