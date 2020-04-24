import { Injectable } from '@nestjs/common';
import { Author } from '@app/ts-interfaces';

@Injectable()
export class AuthorService {
  // TODO: change number to other type when we get rid of int32 in proto
  findOneById(id: number): Author {
    return {
      id,
      name: 'Fake name',
    };
  }
}
