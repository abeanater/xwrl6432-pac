#[doc = "Register `TXESC` reader"]
pub type R = crate::R<TxescSpec>;
#[doc = "Register `TXESC` writer"]
pub type W = crate::W<TxescSpec>;
#[doc = "Field `TBDS` reader - 2:0\\]
Tx Buffer Data Field Size"]
pub type TbdsR = crate::FieldReader;
#[doc = "Field `TBDS` writer - 2:0\\]
Tx Buffer Data Field Size"]
pub type TbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU65` reader - 31:3\\]
Reserved"]
pub type Nu65R = crate::FieldReader<u32>;
#[doc = "Field `NU65` writer - 31:3\\]
Reserved"]
pub type Nu65W<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Tx Buffer Data Field Size"]
    #[inline(always)]
    pub fn tbds(&self) -> TbdsR {
        TbdsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved"]
    #[inline(always)]
    pub fn nu65(&self) -> Nu65R {
        Nu65R::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Tx Buffer Data Field Size"]
    #[inline(always)]
    #[must_use]
    pub fn tbds(&mut self) -> TbdsW<TxescSpec> {
        TbdsW::new(self, 0)
    }
    #[doc = "Bits 3:31 - 31:3\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu65(&mut self) -> Nu65W<TxescSpec> {
        Nu65W::new(self, 3)
    }
}
#[doc = "TXESC\n\nYou can [`read`](crate::Reg::read) this register and get [`txesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxescSpec;
impl crate::RegisterSpec for TxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txesc::R`](R) reader structure"]
impl crate::Readable for TxescSpec {}
#[doc = "`write(|w| ..)` method takes [`txesc::W`](W) writer structure"]
impl crate::Writable for TxescSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXESC to value 0"]
impl crate::Resettable for TxescSpec {
    const RESET_VALUE: u32 = 0;
}
