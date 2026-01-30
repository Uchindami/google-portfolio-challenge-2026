import React from "react";

export const BlogFooter: React.FC = () => {
  const currentYear = new Date().getFullYear();

  const footerLinks = [
    { href: "/web/", text: "Home" },
    { href: "/web/#about", text: "About" },
    { href: "/web/#contact", text: "Get In touch" },
  ];

  return (
    <footer className="bg-gray-100 py-6 mt-8">
      <div className="container mx-auto px-4">
        <div className="flex flex-col md:flex-row justify-between items-center">
          <div className="mb-4 md:mb-0">
            <p className="text-sm text-gray-600">
              © {currentYear} Uchindami. All rights reserved.
            </p>
          </div>
          <nav>
            <ul className="flex space-x-4">
              {footerLinks.map((link) => (
                <li key={link.href}>
                  <a
                    href={link.href}
                    className="text-sm text-gray-600 hover:text-gray-900"
                  >
                    {link.text}
                  </a>
                </li>
              ))}
            </ul>
          </nav>
        </div>
      </div>
    </footer>
  );
};
