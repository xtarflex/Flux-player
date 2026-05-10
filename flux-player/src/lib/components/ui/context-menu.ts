export interface MenuItem {
  label?: string;
  action?: () => void;
  children?: MenuItem[];
  separator?: boolean;
  danger?: boolean;
  isToggle?: boolean;
  toggleValue?: boolean;
  onToggle?: (value: boolean) => void;
}
