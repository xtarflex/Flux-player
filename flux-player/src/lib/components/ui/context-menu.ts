export interface MenuItem {
  label?: string;
  action?: () => void;
  children?: MenuItem[];
  separator?: boolean;
  danger?: boolean;
}
