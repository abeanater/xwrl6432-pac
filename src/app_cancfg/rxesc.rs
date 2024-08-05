#[doc = "Register `RXESC` reader"]
pub type R = crate::R<RxescSpec>;
#[doc = "Register `RXESC` writer"]
pub type W = crate::W<RxescSpec>;
#[doc = "Field `F0DS` reader - 2:0\\]
Rx FIFO 0 Data Field Size"]
pub type F0dsR = crate::FieldReader;
#[doc = "Field `F0DS` writer - 2:0\\]
Rx FIFO 0 Data Field Size"]
pub type F0dsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU56` reader - 3:3\\]
Reserved"]
pub type Nu56R = crate::BitReader;
#[doc = "Field `NU56` writer - 3:3\\]
Reserved"]
pub type Nu56W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `F1DS` reader - 6:4\\]
Rx FIFO 1 Data Field Size"]
pub type F1dsR = crate::FieldReader;
#[doc = "Field `F1DS` writer - 6:4\\]
Rx FIFO 1 Data Field Size"]
pub type F1dsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU57` reader - 7:7\\]
Reserved"]
pub type Nu57R = crate::BitReader;
#[doc = "Field `NU57` writer - 7:7\\]
Reserved"]
pub type Nu57W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBDS` reader - 10:8\\]
Rx Buffer data Field Size"]
pub type RbdsR = crate::FieldReader;
#[doc = "Field `RBDS` writer - 10:8\\]
Rx Buffer data Field Size"]
pub type RbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU58` reader - 31:11\\]
Reserved"]
pub type Nu58R = crate::FieldReader<u32>;
#[doc = "Field `NU58` writer - 31:11\\]
Reserved"]
pub type Nu58W<'a, REG> = crate::FieldWriter<'a, REG, 21, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Rx FIFO 0 Data Field Size"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0dsR {
        F0dsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - 3:3\\]
Reserved"]
    #[inline(always)]
    pub fn nu56(&self) -> Nu56R {
        Nu56R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Rx FIFO 1 Data Field Size"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1dsR {
        F1dsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved"]
    #[inline(always)]
    pub fn nu57(&self) -> Nu57R {
        Nu57R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Rx Buffer data Field Size"]
    #[inline(always)]
    pub fn rbds(&self) -> RbdsR {
        RbdsR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved"]
    #[inline(always)]
    pub fn nu58(&self) -> Nu58R {
        Nu58R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Rx FIFO 0 Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn f0ds(&mut self) -> F0dsW<RxescSpec> {
        F0dsW::new(self, 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu56(&mut self) -> Nu56W<RxescSpec> {
        Nu56W::new(self, 3)
    }
    #[doc = "Bits 4:6 - 6:4\\]
Rx FIFO 1 Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn f1ds(&mut self) -> F1dsW<RxescSpec> {
        F1dsW::new(self, 4)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu57(&mut self) -> Nu57W<RxescSpec> {
        Nu57W::new(self, 7)
    }
    #[doc = "Bits 8:10 - 10:8\\]
Rx Buffer data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn rbds(&mut self) -> RbdsW<RxescSpec> {
        RbdsW::new(self, 8)
    }
    #[doc = "Bits 11:31 - 31:11\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu58(&mut self) -> Nu58W<RxescSpec> {
        Nu58W::new(self, 11)
    }
}
#[doc = "RXESC\n\nYou can [`read`](crate::Reg::read) this register and get [`rxesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxescSpec;
impl crate::RegisterSpec for RxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxesc::R`](R) reader structure"]
impl crate::Readable for RxescSpec {}
#[doc = "`write(|w| ..)` method takes [`rxesc::W`](W) writer structure"]
impl crate::Writable for RxescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXESC to value 0"]
impl crate::Resettable for RxescSpec {
    const RESET_VALUE: u32 = 0;
}
