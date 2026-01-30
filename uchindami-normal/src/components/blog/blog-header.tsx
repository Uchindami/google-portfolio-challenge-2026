import "../../styles/global.css";
import { Menu, Search } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet";
import { Navigation } from "@/components/pages/Navigation";
import { toast } from "sonner";
import SearchBox from "@/components/search-box.tsx";

export default function BlogHeader({ allPosts }) {
  const handleSearch = () => {
    toast.info("Search is currently only supported on PC");
  };

  return (
    <header className="w-full m-0 p-0">
      <div className="container !py-0 px-4 sm:px-6 lg:px-8">
        <div className="flex h-16 items-center justify-between">
          {/* Logo */}
          <div className="flex-shrink-0">
            <a href="/web/" className="text-xl font-bold text-blog-primary">
              Uchindami
            </a>
          </div>

          {/* Search button (hidden on mobile) */}
          <div className="hidden sm:block">
            <SearchBox articleData={allPosts} />
          </div>

          {/* Navigation links (hidden on mobile) */}
          <nav className="hidden md:flex items-center space-x-4">
            <Navigation />
          </nav>

          {/* Search button (visible only on mobile) */}
          <div className="sm:hidden">
            <Button variant="outline" size="icon" onClick={handleSearch}>
              <Search className="h-4 w-4" />
              <span className="sr-only">Search</span>
            </Button>
          </div>

          {/* Mobile menu button */}
          <div className="flex md:hidden">
            <Sheet>
              <SheetTrigger asChild>
                <Button variant="outline" size="icon">
                  <Menu className="h-4 w-4" />
                  <span className="sr-only">Open menu</span>
                </Button>
              </SheetTrigger>
              <SheetContent side="right">
                <nav className="flex flex-col space-y-4 mt-4">
                  <Navigation />
                </nav>
              </SheetContent>
            </Sheet>
          </div>
        </div>
      </div>
    </header>
  );
}
