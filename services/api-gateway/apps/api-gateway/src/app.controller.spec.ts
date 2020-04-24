import { Test, TestingModule } from '@nestjs/testing';
import { AppController } from './app.controller';
import { AppService } from './app.service';
import { of } from 'rxjs';

describe('AppController', () => {
  let appController: AppController;
  const mockedAppService = {
    getAuthor: () => of({ id: 42, name: 'test' }),
  };

  beforeEach(async () => {
    const app: TestingModule = await Test.createTestingModule({
      controllers: [AppController],
      providers: [AppService],
    })
      .overrideProvider(AppService)
      .useValue(mockedAppService)
      .compile();

    appController = app.get<AppController>(AppController);
  });

  describe('root', () => {
    it('should return "Hello World!"', async () => {
      expect(await appController.getAuthor().toPromise()).toEqual({ id: 42, name: 'test' });
    });
  });
});
