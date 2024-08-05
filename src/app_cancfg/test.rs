#[doc = "Register `TEST` reader"]
pub type R = crate::R<TestSpec>;
#[doc = "Register `TEST` writer"]
pub type W = crate::W<TestSpec>;
#[doc = "Field `NU14` reader - 3:0\\]
Reserved"]
pub type Nu14R = crate::FieldReader;
#[doc = "Field `NU14` writer - 3:0\\]
Reserved"]
pub type Nu14W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LBCK` reader - 4:4\\]
Loop Back Mode"]
pub type LbckR = crate::BitReader;
#[doc = "Field `LBCK` writer - 4:4\\]
Loop Back Mode"]
pub type LbckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX` reader - 6:5\\]
Control of Transmit Pin"]
pub type TxR = crate::FieldReader;
#[doc = "Field `TX` writer - 6:5\\]
Control of Transmit Pin"]
pub type TxW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RX` reader - 7:7\\]
Receive Pin"]
pub type RxR = crate::BitReader;
#[doc = "Field `RX` writer - 7:7\\]
Receive Pin"]
pub type RxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU15` reader - 31:8\\]
Reserved"]
pub type Nu15R = crate::FieldReader<u32>;
#[doc = "Field `NU15` writer - 31:8\\]
Reserved"]
pub type Nu15W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Reserved"]
    #[inline(always)]
    pub fn nu14(&self) -> Nu14R {
        Nu14R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - 4:4\\]
Loop Back Mode"]
    #[inline(always)]
    pub fn lbck(&self) -> LbckR {
        LbckR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Control of Transmit Pin"]
    #[inline(always)]
    pub fn tx(&self) -> TxR {
        TxR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Pin"]
    #[inline(always)]
    pub fn rx(&self) -> RxR {
        RxR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn nu15(&self) -> Nu15R {
        Nu15R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu14(&mut self) -> Nu14W<TestSpec> {
        Nu14W::new(self, 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Loop Back Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lbck(&mut self) -> LbckW<TestSpec> {
        LbckW::new(self, 4)
    }
    #[doc = "Bits 5:6 - 6:5\\]
Control of Transmit Pin"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TxW<TestSpec> {
        TxW::new(self, 5)
    }
    #[doc = "Bit 7 - 7:7\\]
Receive Pin"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RxW<TestSpec> {
        RxW::new(self, 7)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu15(&mut self) -> Nu15W<TestSpec> {
        Nu15W::new(self, 8)
    }
}
#[doc = "TEST\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TestSpec;
impl crate::RegisterSpec for TestSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test::R`](R) reader structure"]
impl crate::Readable for TestSpec {}
#[doc = "`write(|w| ..)` method takes [`test::W`](W) writer structure"]
impl crate::Writable for TestSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TEST to value 0"]
impl crate::Resettable for TestSpec {
    const RESET_VALUE: u32 = 0;
}
