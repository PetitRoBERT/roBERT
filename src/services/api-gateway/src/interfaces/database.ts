import { Observable } from 'rxjs';
import { AuthorById } from '../author/interfaces/author-by-id';
import { Author } from '../author/interfaces/author';

// TODO: export multiple services' interface for database (Author, Book, ...)
// Yhen move this in author/interfaces
export interface DatabaseService {
    findOneAuthorById(authorById: AuthorById): Observable<Author>;
}