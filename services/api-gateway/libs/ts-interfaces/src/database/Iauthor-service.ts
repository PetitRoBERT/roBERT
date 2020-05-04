import { Metadata } from 'grpc';
import { Observable } from 'rxjs';

import { IAuthor } from './Iauthor';
import { IAuthorById } from './Iauthor-by-id';

export interface IAuthorService {
  findOneById(request: IAuthorById, metadata?: Metadata): Observable<IAuthor>;
}
