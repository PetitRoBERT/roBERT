import { Metadata } from "grpc";
import { Observable } from "rxjs";

import { Author } from "./author";
import { AuthorById } from "./author-by-id";

export interface AuthorService {
  findOneById(request: AuthorById, metadata?: Metadata): Observable<Author>;
}
