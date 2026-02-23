import { buttonVariants } from "./button-variants";
import { cn } from "../../../lib/utils";

export const LinkButton = ({
  className,
  variant,
  size,
  href,
  ...props
}: React.AnchorHTMLAttributes<HTMLAnchorElement> & any) => {
  return (
    <a
      href={href}
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  );
};
