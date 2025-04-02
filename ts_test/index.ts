import { OkId, Uuid } from "okid";

export const okid = new Uuid().into_okid();

console.log(okid.display_safe());
