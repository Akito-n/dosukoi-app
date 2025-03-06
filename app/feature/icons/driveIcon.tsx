import { ImDrive } from "react-icons/im";

type EditIconProps = {
  size?: number
  className?: string
}

const DriveIcon = ({ size, className }: EditIconProps) => {
  return <ImDrive  size={size} className={className} />
}

export default DriveIcon
