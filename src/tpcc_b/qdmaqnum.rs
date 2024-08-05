#[doc = "Register `QDMAQNUM` reader"]
pub type R = crate::R<QdmaqnumSpec>;
#[doc = "Register `QDMAQNUM` writer"]
pub type W = crate::W<QdmaqnumSpec>;
#[doc = "Field `E0` reader - 2:0\\]
QDMA Queue Number for event #0"]
pub type E0R = crate::FieldReader;
#[doc = "Field `E0` writer - 2:0\\]
QDMA Queue Number for event #0"]
pub type E0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES26` reader - 3:3\\]
RESERVE FIELD"]
pub type Res26R = crate::BitReader;
#[doc = "Field `RES26` writer - 3:3\\]
RESERVE FIELD"]
pub type Res26W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E1` reader - 6:4\\]
QDMA Queue Number for event #1"]
pub type E1R = crate::FieldReader;
#[doc = "Field `E1` writer - 6:4\\]
QDMA Queue Number for event #1"]
pub type E1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES25` reader - 7:7\\]
RESERVE FIELD"]
pub type Res25R = crate::BitReader;
#[doc = "Field `RES25` writer - 7:7\\]
RESERVE FIELD"]
pub type Res25W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E2` reader - 10:8\\]
QDMA Queue Number for event #2"]
pub type E2R = crate::FieldReader;
#[doc = "Field `E2` writer - 10:8\\]
QDMA Queue Number for event #2"]
pub type E2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES24` reader - 11:11\\]
RESERVE FIELD"]
pub type Res24R = crate::BitReader;
#[doc = "Field `RES24` writer - 11:11\\]
RESERVE FIELD"]
pub type Res24W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E3` reader - 14:12\\]
QDMA Queue Number for event #3"]
pub type E3R = crate::FieldReader;
#[doc = "Field `E3` writer - 14:12\\]
QDMA Queue Number for event #3"]
pub type E3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES23` reader - 15:15\\]
RESERVE FIELD"]
pub type Res23R = crate::BitReader;
#[doc = "Field `RES23` writer - 15:15\\]
RESERVE FIELD"]
pub type Res23W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E4` reader - 18:16\\]
QDMA Queue Number for event #4"]
pub type E4R = crate::FieldReader;
#[doc = "Field `E4` writer - 18:16\\]
QDMA Queue Number for event #4"]
pub type E4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES22` reader - 19:19\\]
RESERVE FIELD"]
pub type Res22R = crate::BitReader;
#[doc = "Field `RES22` writer - 19:19\\]
RESERVE FIELD"]
pub type Res22W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E5` reader - 22:20\\]
QDMA Queue Number for event #5"]
pub type E5R = crate::FieldReader;
#[doc = "Field `E5` writer - 22:20\\]
QDMA Queue Number for event #5"]
pub type E5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES21` reader - 23:23\\]
RESERVE FIELD"]
pub type Res21R = crate::BitReader;
#[doc = "Field `RES21` writer - 23:23\\]
RESERVE FIELD"]
pub type Res21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E6` reader - 26:24\\]
QDMA Queue Number for event #6"]
pub type E6R = crate::FieldReader;
#[doc = "Field `E6` writer - 26:24\\]
QDMA Queue Number for event #6"]
pub type E6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES20` reader - 27:27\\]
RESERVE FIELD"]
pub type Res20R = crate::BitReader;
#[doc = "Field `RES20` writer - 27:27\\]
RESERVE FIELD"]
pub type Res20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `E7` reader - 30:28\\]
QDMA Queue Number for event #7"]
pub type E7R = crate::FieldReader;
#[doc = "Field `E7` writer - 30:28\\]
QDMA Queue Number for event #7"]
pub type E7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RES19` reader - 31:31\\]
RESERVE FIELD"]
pub type Res19R = crate::BitReader;
#[doc = "Field `RES19` writer - 31:31\\]
RESERVE FIELD"]
pub type Res19W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
QDMA Queue Number for event #0"]
    #[inline(always)]
    pub fn e0(&self) -> E0R {
        E0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res26(&self) -> Res26R {
        Res26R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
QDMA Queue Number for event #1"]
    #[inline(always)]
    pub fn e1(&self) -> E1R {
        E1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res25(&self) -> Res25R {
        Res25R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
QDMA Queue Number for event #2"]
    #[inline(always)]
    pub fn e2(&self) -> E2R {
        E2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 11:11\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res24(&self) -> Res24R {
        Res24R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - 14:12\\]
QDMA Queue Number for event #3"]
    #[inline(always)]
    pub fn e3(&self) -> E3R {
        E3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res23(&self) -> Res23R {
        Res23R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
QDMA Queue Number for event #4"]
    #[inline(always)]
    pub fn e4(&self) -> E4R {
        E4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - 19:19\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res22(&self) -> Res22R {
        Res22R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - 22:20\\]
QDMA Queue Number for event #5"]
    #[inline(always)]
    pub fn e5(&self) -> E5R {
        E5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - 23:23\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res21(&self) -> Res21R {
        Res21R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - 26:24\\]
QDMA Queue Number for event #6"]
    #[inline(always)]
    pub fn e6(&self) -> E6R {
        E6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - 27:27\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res20(&self) -> Res20R {
        Res20R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - 30:28\\]
QDMA Queue Number for event #7"]
    #[inline(always)]
    pub fn e7(&self) -> E7R {
        E7R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - 31:31\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res19(&self) -> Res19R {
        Res19R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
QDMA Queue Number for event #0"]
    #[inline(always)]
    #[must_use]
    pub fn e0(&mut self) -> E0W<QdmaqnumSpec> {
        E0W::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res26(&mut self) -> Res26W<QdmaqnumSpec> {
        Res26W::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
QDMA Queue Number for event #1"]
    #[inline(always)]
    #[must_use]
    pub fn e1(&mut self) -> E1W<QdmaqnumSpec> {
        E1W::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res25(&mut self) -> Res25W<QdmaqnumSpec> {
        Res25W::new(self, 7)
    }
    #[doc = "Bits 8:10 - 10:8\\]
QDMA Queue Number for event #2"]
    #[inline(always)]
    #[must_use]
    pub fn e2(&mut self) -> E2W<QdmaqnumSpec> {
        E2W::new(self, 8)
    }
    #[doc = "Bit 11 - 11:11\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res24(&mut self) -> Res24W<QdmaqnumSpec> {
        Res24W::new(self, 11)
    }
    #[doc = "Bits 12:14 - 14:12\\]
QDMA Queue Number for event #3"]
    #[inline(always)]
    #[must_use]
    pub fn e3(&mut self) -> E3W<QdmaqnumSpec> {
        E3W::new(self, 12)
    }
    #[doc = "Bit 15 - 15:15\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res23(&mut self) -> Res23W<QdmaqnumSpec> {
        Res23W::new(self, 15)
    }
    #[doc = "Bits 16:18 - 18:16\\]
QDMA Queue Number for event #4"]
    #[inline(always)]
    #[must_use]
    pub fn e4(&mut self) -> E4W<QdmaqnumSpec> {
        E4W::new(self, 16)
    }
    #[doc = "Bit 19 - 19:19\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res22(&mut self) -> Res22W<QdmaqnumSpec> {
        Res22W::new(self, 19)
    }
    #[doc = "Bits 20:22 - 22:20\\]
QDMA Queue Number for event #5"]
    #[inline(always)]
    #[must_use]
    pub fn e5(&mut self) -> E5W<QdmaqnumSpec> {
        E5W::new(self, 20)
    }
    #[doc = "Bit 23 - 23:23\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res21(&mut self) -> Res21W<QdmaqnumSpec> {
        Res21W::new(self, 23)
    }
    #[doc = "Bits 24:26 - 26:24\\]
QDMA Queue Number for event #6"]
    #[inline(always)]
    #[must_use]
    pub fn e6(&mut self) -> E6W<QdmaqnumSpec> {
        E6W::new(self, 24)
    }
    #[doc = "Bit 27 - 27:27\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res20(&mut self) -> Res20W<QdmaqnumSpec> {
        Res20W::new(self, 27)
    }
    #[doc = "Bits 28:30 - 30:28\\]
QDMA Queue Number for event #7"]
    #[inline(always)]
    #[must_use]
    pub fn e7(&mut self) -> E7W<QdmaqnumSpec> {
        E7W::new(self, 28)
    }
    #[doc = "Bit 31 - 31:31\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res19(&mut self) -> Res19W<QdmaqnumSpec> {
        Res19W::new(self, 31)
    }
}
#[doc = "QDMA Queue Number Register Contains the Event queue number to be used for the corresponding QDMA Channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`qdmaqnum::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qdmaqnum::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QdmaqnumSpec;
impl crate::RegisterSpec for QdmaqnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qdmaqnum::R`](R) reader structure"]
impl crate::Readable for QdmaqnumSpec {}
#[doc = "`write(|w| ..)` method takes [`qdmaqnum::W`](W) writer structure"]
impl crate::Writable for QdmaqnumSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QDMAQNUM to value 0"]
impl crate::Resettable for QdmaqnumSpec {
    const RESET_VALUE: u32 = 0;
}
