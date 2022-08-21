"""
Create a set unit test for rsplitter
"""
import unittest
import time

import rsplitter

TEXTS = [
    ("Thequickbrownfoxjumpsoverthelazydog", "The quick brown fox jumps over the lazy dog"),
    ("rustisveryfast", "rust is very fast"),
    ("pythonisfast", "python is fast"),
    ("javaisjustbad", "java is just bad"),
    ("cisgreat", "c is great"),
    ("c++isc++", "c++ is c++"),
]


class TestRsplitter(unittest.TestCase):

    def test_default(self):
        """
        Test the default rsplitter
        """
        self.assertEqual(rsplitter.split("Thequickbrownfoxjumpsoverthelazydog"), 'The quick brown fox jumps over the lazy dog')


    def test_custom_language_model(self):
        """
        Test the custom language model
        """
        language_model = rsplitter.LanguageModel('src/corpus.txt')
        self.assertEqual(language_model.split("Thequickbrownfoxjumpsoverthelazydog"), 'The quick brown fox jumps over the lazy dog')

    def test_default_execution_speed(self):
        """
        Test the default execution speed
        """
        start = time.perf_counter()
        rsplitter.split("Thequickbrownfoxjumpsoverthelazydog")
        end = time.perf_counter()
        self.assertLessEqual(end - start, 0.1)

    def test_custom_language_model_execution_speed(self):
        """
        Test the custom language model execution speed
        """
        language_model = rsplitter.LanguageModel('src/corpus.txt')
        start = time.perf_counter()
        language_model.split("Thequickbrownfoxjumpsoverthelazydog")
        end = time.perf_counter()
        self.assertLessEqual(end - start, 0.1)


if __name__ == '__main__':
    unittest.main()
