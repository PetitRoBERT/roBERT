import { Test, TestingModule } from '@nestjs/testing';
import { AuthorController } from './author.controller';
import { AuthorService } from './author.service';

describe('AuthorController', () => {
  let authorController: AuthorController;

  beforeEach(async () => {
    const app: TestingModule = await Test.createTestingModule({
      controllers: [AuthorController],
      providers: [AuthorService],
    }).compile();

    authorController = app.get<AuthorController>(AuthorController);
  });

  describe('root', () => {
    it('should return "Hello World!"', () => {
      expect(authorController.findOneById({id: 0})).toBe({
        id: 0,
        name: 'Fake name'
      });
    });
  });
});
